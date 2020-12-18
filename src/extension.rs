use std::{
	cmp::Ordering,
	ops::Deref,
	hash::{
		Hash,
		Hasher
	},
	fmt,
	convert::TryFrom
};
use crate::{
	component,
	iterator,
	parse,
	Error
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Singleton(u8);

impl Singleton {
	#[inline]
	pub fn unwrap(self) -> u8 {
		self.0
	}
}

impl TryFrom<u8> for Singleton {
	type Error = Error;

	#[inline]
	fn try_from(b: u8) -> Result<Singleton, Error> {
		if parse::is_singleton(b) {
			Ok(Singleton(b))
		} else {
			Err(Error::InvalidSingleton(b))
		}
	}
}

impl TryFrom<char> for Singleton {
	type Error = Error;

	#[inline]
	fn try_from(c: char) -> Result<Singleton, Error> {
		let codepoint = c as u32;

		if codepoint <= 0xff && parse::is_singleton(codepoint as u8) {
			Ok(Singleton(codepoint as u8))
		} else {
			Err(Error::InvalidCharSingleton(c))
		}
	}
}

impl PartialEq<u8> for Singleton {
	fn eq(&self, b: &u8) -> bool {
		self.0 == *b
	}
}

impl PartialEq<char> for Singleton {
	fn eq(&self, b: &char) -> bool {
		self.0 as u32 == *b as u32
	}
}

impl fmt::Display for Singleton {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(self.0 as char).fmt(f)
	}
}

impl fmt::Debug for Singleton {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(self.0 as char).fmt(f)
	}
}

component! {
	/// Single extension and its subtags.
	/// 
	/// Extensions provide a mechanism for extending language tags for use in
	/// various applications. They are intended to identify information that
	/// is commonly used in association with languages or language tags but
	/// that is not part of language identification.
	/// 
	/// An extension is composed of a singleton (a single character)
	/// followed by associated subtags.
	/// For instance `a-subtag1-subtag2`.
	/// Each subtag of the extension is represented by the [`ExtensionSubtag`] type.
	extension, false, Extension, InvalidExtension
}

component! {
	/// Single extension subtag.
	/// 
	/// Extension subtag found in a language tag extension.
	extension_subtag, false, ExtensionSubtag, InvalidExtensionSubtag
}

component! {
	/// List of extension subtags.
	/// 
	/// A list of language tag extension, separated by a `-` character.
	/// Individual extensions are represented by the [`Extension`] type,
	/// while extension subtags are represented by the [`ExtensionSubtag`]
	/// type.
	extensions, true, Extensions, InvalidExtensions
}

iterator!(Extension, ExtensionIter, ExtensionSubtag, 2);

impl Extension {
	/// Return the singletong identifying the extension.
	pub fn singleton(&self) -> u8 {
		self.data[0]
	}
}

impl Extensions {
	/// The empty list of extension (an empty string).
	pub fn empty() -> &'static Extensions {
		unsafe {
			Extensions::parse_unchecked(b"")
		}
	}

	/// Iterate extensions.
	pub fn iter(&self) -> ExtensionsIter {
		ExtensionsIter {
			bytes: &self.data,
			i: 0
		}
	}
	
	/// Iterate through each extensions subtags.
	/// 
	/// Eah subtag will be attached to its extension's singleton.
	pub fn iter_subtags(&self) -> ExtensionsSubtagsIter {
		ExtensionsSubtagsIter {
			bytes: &self.data,
			i: 0,
			current_id: Singleton(0) // dummy singleton tht is never used.
		}
	}
	
	/// Iterate through a specific extension subtags.
	pub fn iter_extension(&self, e: Singleton) -> ExtensionSubtagsIter {
		ExtensionSubtagsIter {
			it: self.iter_subtags(),
			extension: e
		}
	}
}

impl<'a> IntoIterator for &'a Extensions {
	type IntoIter = ExtensionsIter<'a>;
	type Item = &'a Extension;

	fn into_iter(self) -> ExtensionsIter<'a> {
		ExtensionsIter {
			bytes: &self.data,
			i: 0
		}
	}
}

/// Extensions iterator.
pub struct ExtensionsIter<'a> {
	bytes: &'a [u8],
	i: usize,
}

impl<'a> Iterator for ExtensionsIter<'a> {
	type Item = &'a Extension;

	fn next(&mut self) -> Option<Self::Item> {
		if self.i < self.bytes.len() {
			let offset = self.i;
			let mut j = self.i;

			loop {
				if self.i < self.bytes.len() {
					if self.bytes[self.i] == b'-' {
						if j+2 == self.i {
							self.i = j+1;
							break
						} else {
							j = self.i;
						}
					}
	
					self.i += 1;
				} else {
					j = self.i;
					break
				}
			}

			unsafe {
				return Some(Extension::parse_unchecked(&self.bytes[offset..j]))
			}
		} else {
			None
		}
	}
}

/// Extensions subtags iterator.
pub struct ExtensionsSubtagsIter<'a> {
	bytes: &'a [u8],
	i: usize,
	current_id: Singleton
}

impl<'a> Iterator for ExtensionsSubtagsIter<'a> {
	type Item = (Singleton, &'a ExtensionSubtag);

	fn next(&mut self) -> Option<Self::Item> {
		if self.i < self.bytes.len() {
			loop {
				if self.bytes[self.i] == b'-' {
					self.i += 1;
				}

				let offset = self.i;

				while self.i < self.bytes.len() && self.bytes[self.i] != b'-' {
					self.i += 1;
				}

				if self.i > offset+1 {
					unsafe {
						return Some((self.current_id, ExtensionSubtag::parse_unchecked(&self.bytes[offset..self.i])))
					}
				} else {
					self.current_id = Singleton(self.bytes[offset]);
				}
			}
		} else {
			None
		}
	}
}

/// Extension subtags iterator.
pub struct ExtensionSubtagsIter<'a> {
	it: ExtensionsSubtagsIter<'a>,
	extension: Singleton
}

impl<'a> Iterator for ExtensionSubtagsIter<'a> {
	type Item = &'a ExtensionSubtag;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some((e, tag)) = self.it.next() {
			if e == self.extension {
				return Some(tag)
			}
		}

		None
	}
}

pub struct ExtensionsMut<'a> {
	/// Language tag buffer.
	pub(crate) buffer: &'a mut Vec<u8>,

	/// Language tag parsing data.
	pub(crate) p: &'a mut parse::ParsedLangTag
}

impl<'a> ExtensionsMut<'a> {
	pub fn insert(&mut self, singleton: Singleton, subtag: &ExtensionSubtag) {
		let mut i = self.p.variant_end+1;
		let mut subtag_offset = i;
		let mut insert_offset = None;
		let mut current_singleton = 0;

		while i < self.p.extension_end {
			if self.buffer[i] == b'-' {
				if subtag_offset+1 == i {
					current_singleton = self.buffer[subtag_offset];
				}

				if singleton == current_singleton {
					insert_offset = Some(i);
				}

				subtag_offset = i+1;
			}

			i += 1;
		}

		if i == self.p.extension_end && singleton == current_singleton {
			insert_offset = Some(i);
		}

		let len = match insert_offset {
			Some(i) => {
				crate::replace(self.buffer, i..i, subtag.as_ref());
				crate::replace(self.buffer, i..i, b"-");
				subtag.len() + 1
			},
			None => {
				let i = self.p.extension_end;
				crate::replace(self.buffer, i..i, subtag.as_ref());
				crate::replace(self.buffer, i..i, &[b'-', singleton.unwrap(), b'-']);
				subtag.len() + 3
			}
		};

		self.p.extension_end += len;
		self.p.privateuse_end += len;
	}

	/// Remove the extension identified by the given singleton.
	pub fn remove(&mut self, singleton: Singleton) -> bool {
		let mut i = self.p.variant_end+1;
		let mut subtag_offset = i;
		let mut extension_offset = None;
		let mut removed = false;

		while i < self.p.extension_end {
			if self.buffer[i] == b'-' {
				if subtag_offset+1 == i {
					if singleton == self.buffer[subtag_offset] {
						if extension_offset.is_none() {
							extension_offset = Some(subtag_offset-1);
						}
					} else {
						if let Some(offset) = extension_offset {
							let len = subtag_offset-1-offset;
							crate::replace(self.buffer, offset..(subtag_offset-1), &[]);
							self.p.extension_end -= len;
							self.p.privateuse_end -= len;
							i = offset+2;
							extension_offset = None;
							removed = true
						}
					}
				}

				subtag_offset = i+1
			}

			i += 1;
		}

		if let Some(offset) = extension_offset {
			let end = self.p.extension_end;
			let len = end-offset;
			crate::replace(self.buffer, offset..end, &[]);
			self.p.extension_end -= len;
			self.p.privateuse_end -= len;
			removed = true
		}

		removed
	}

	pub fn remove_subtag<T: AsRef<[u8]> + ?Sized>(&mut self, singleton: Singleton, subtag: &T) -> bool {
		let subtag = subtag.as_ref();
		let mut i = self.p.variant_end+1;
		let mut subtag_offset = i;
		let mut scope_offset = 0;
		let mut in_scope = false;
		let mut removed = false;

		while i < self.p.extension_end {
			if self.buffer[i] == b'-' {
				if subtag_offset+1 == i {
					in_scope = singleton == self.buffer[subtag_offset];
					scope_offset = subtag_offset;
				} else if in_scope && &self.buffer[subtag_offset..i] == subtag {
					let offset = if subtag_offset == scope_offset+2 && i+2 < self.p.extension_end && self.buffer[i+2] == b'-' {
						subtag_offset-3
					} else {
						subtag_offset-1
					};
					let len = i-offset;
					crate::replace(self.buffer, offset..i, &[]);
					self.p.extension_end -= len;
					self.p.privateuse_end -= len;
					i -= len;
					removed = true
				}

				subtag_offset = i+1
			}

			i += 1;
		}

		if in_scope && &self.buffer[subtag_offset..i] == subtag {
			let offset = if subtag_offset == scope_offset+2 {
				subtag_offset-3
			} else {
				subtag_offset-1
			};
			let len = i-offset;
			crate::replace(self.buffer, offset..i, &[]);
			self.p.extension_end -= len;
			self.p.privateuse_end -= len;
			removed = true
		}

		removed
	}
}