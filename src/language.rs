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
	parse,
	Error
};

component! {
	/// Primary and extended language subtags.
	/// 
	/// This type represents the primary language subtag (first subtag in a
	/// language tag) and the extended language subtags associated with it.
	language, false, Language, InvalidLanguage
}

component! {
	/// Primary language subtag.
	/// 
	/// The primary language subtag is the first subtag in a language tag.
	primary_language, false, PrimaryLanguage, InvalidPrimaryLanguage
}

component! {
	/// List of extended language subtags.
	/// 
	/// This type represents a list of extended language subtags,
	/// separated by a `-` character.
	/// 
	/// Extended language subtags are used to identify certain specially
	/// selected languages that, for various historical and compatibility
	/// reasons, are closely identified with or tagged using an existing
	/// primary language subtag.
	/// The type [`ExtendedLangTag`] represents a single extended
	/// language subtag.
	extlang, false, LanguageExtension, InvalidLanguageExtension
}

component! {
	/// Single extended language subtag.
	/// 
	/// Extended language subtags are used to identify certain specially
	/// selected languages that, for various historical and compatibility
	/// reasons, are closely identified with or tagged using an existing
	/// primary language subtag.
	/// 
	/// The type [`LanguageExtension`] represents a list of
	/// extended language.
	extlang_tag, false, ExtendedLangTag, InvalidExtendedLangTag
}

impl<T: AsRef<[u8]> + ?Sized> LanguageExtension<T> {
	/// Checks if more extended language subtags can be added.
	fn is_full(&self) -> bool {
		self.len() >= 11
	}
}

iterator!(LanguageExtension, LanguageExtensionIter, ExtendedLangTag, 0);

impl Language {
	/// Return the length (in bytes) of the primary subtag.
	#[inline]
	fn primary_len(&self) -> usize {
		let bytes = self.as_bytes();
		let mut i = 0;

		while i < bytes.len() {
			if bytes[i] == b'-' {
				break
			}

			i += 1;
		}

		i
	}

	/// Return the primary language subtag.
	/// 
	/// The primary language subtag is the first subtag in a language tag.
	#[inline]
	pub fn primary(&self) -> &PrimaryLanguage {
		unsafe {
			PrimaryLanguage::parse_unchecked(&self.as_bytes()[..self.primary_len()])
		}
	}

	/// Return the extended language subtags.
	/// 
	/// Extended language subtags are used to identify certain specially
	/// selected languages that, for various historical and compatibility
	/// reasons, are closely identified with or tagged using an existing
	/// primary language subtag.
	/// 
	/// Extended language subtags are only present when the primary
	/// language subtag length is 2 or 3.
	#[inline]
	pub fn extension(&self) -> Option<&LanguageExtension> {
		let bytes = self.as_bytes();
		let primary_len = self.primary_len();
		if primary_len < 4 {
			let i = primary_len+1;
			if i < bytes.len() {
				unsafe {
					Some(LanguageExtension::parse_unchecked(&self.as_bytes()[i..]))
				}
			} else {
				Some(LanguageExtension::empty())
			}
		} else {
			None
		}
	}

	/// Return an iterator to the extended language subtags.
	#[inline]
	pub fn extension_subtags(&self) -> LanguageExtensionIter {
		let offset = self.primary_len()+1;
		let bytes = self.as_bytes();
		LanguageExtensionIter {
			bytes: if offset < bytes.len() {
				&bytes[(self.primary_len()+1)..]
			} else {
				&[]
			},
			i: 0
		}
	}
}

/// Mutable reference to language subtags.
pub struct LanguageMut<'a> {
	/// Language tag buffer.
	pub(crate) buffer: &'a mut Vec<u8>,

	/// Language tag parsing data.
	pub(crate) p: &'a mut parse::ParsedLangTag
}

impl<'a> LanguageMut<'a> {
	#[inline]
	pub fn as_ref(&self) -> &Language {
		unsafe {
			Language::parse_unchecked(&self.buffer[0..self.p.language_end])
		}
	}

	/// Get the primary language subtag.
	#[inline]
	pub fn primary(&self) -> &PrimaryLanguage {
		self.as_ref().primary()
	}

	/// Set the primary language subtag.
	/// 
	/// Note that no extended language subtags can be defined if the
	/// primary language subtag's length is greater than 3.
	/// If a new primary language subtag of length 4 or more is defined,
	/// extended language subtags are removed.
	#[inline]
	pub fn set_primary(&mut self, primary: &PrimaryLanguage) {
		let primary = primary.as_bytes();
		let (len, new_end) = if primary.len() > 4 {
			(self.p.language_end, primary.len())
		} else {
			let primary_len = self.as_ref().primary_len();
			let ext_len = self.as_ref().len() - primary_len;
			(primary_len, primary.len() + ext_len)
		};

		crate::replace(self.buffer, 0..len, primary);
		
		if self.p.language_end < new_end {
			let diff = new_end - self.p.language_end;
			self.p.script_end += diff;
			self.p.region_end += diff;
			self.p.variant_end += diff;
			self.p.extension_end += diff;
			self.p.privateuse_end += diff;
		} else {
			let diff = self.p.language_end - new_end;
			self.p.script_end -= diff;
			self.p.region_end -= diff;
			self.p.variant_end -= diff;
			self.p.extension_end -= diff;
			self.p.privateuse_end -= diff;
		}
		self.p.language_end = new_end;
	}
	
	/// Get the extended language subtags.
	#[inline]
	pub fn extension(&self) -> Option<&LanguageExtension> {
		self.as_ref().extension()
	}

	/// Set the extended language subtags.
	/// 
	/// Note that no extended language subtags can be defined if the
	/// primary language subtag's length is greater than 3.
	/// This function returns `true` if the extended language subtags
	/// has been set or are `None`,
	/// and `false` if the primary language subtag is of length 4 or
	/// more and the extended language subtags have not been set.
	#[inline]
	pub fn set_extension(&mut self, ext: Option<&LanguageExtension>) -> bool {
		let primary_len = self.as_ref().primary_len();
		let mut new_end = self.p.language_end;
		let replaced = match ext {
			Some(ext) => {
				if primary_len > 4 {
					false
				} else {
					let len = self.p.language_end - primary_len;
					crate::replace(self.buffer, primary_len..self.p.language_end, ext.as_ref());
					crate::replace(self.buffer, primary_len..primary_len, &[b'-']);
					new_end -= len;
					new_end += ext.len() + 1;
					true
				}
			},
			None => {
				let len = self.p.language_end - primary_len;
				crate::replace(self.buffer, primary_len..self.p.language_end, &[]);
				new_end -= len;
				true
			}
		};

		if replaced {
			if self.p.language_end < new_end {
				let diff = new_end - self.p.language_end;
				self.p.script_end += diff;
				self.p.region_end += diff;
				self.p.variant_end += diff;
				self.p.extension_end += diff;
				self.p.privateuse_end += diff;
			} else {
				let diff = self.p.language_end - new_end;
				self.p.script_end -= diff;
				self.p.region_end -= diff;
				self.p.variant_end -= diff;
				self.p.extension_end -= diff;
				self.p.privateuse_end -= diff;
			}
			self.p.language_end = new_end;
		}

		replaced
	}

	/// Get and modify the extended language subtags, if any.
	#[inline]
	pub fn extension_mut(&mut self) -> Option<LanguageExtensionMut> {
		let primary_len = self.as_ref().primary_len();
		if primary_len < 4 {
			Some(LanguageExtensionMut {
				buffer: self.buffer,
				p: self.p
			})
		} else {
			None
		}
	}
}

/// Mutable reference to extended language subtags.
pub struct LanguageExtensionMut<'a> {
	/// Language tag buffer.
	buffer: &'a mut Vec<u8>,

	/// Language tag parsing data.
	p: &'a mut parse::ParsedLangTag
}

impl<'a> LanguageExtensionMut<'a> {
	/// Return the length (in bytes) of the primary subtag.
	#[inline]
	fn primary_len(&self) -> usize {
		let mut i = 0;

		while i < self.p.language_end {
			if self.buffer[i] == b'-' {
				break
			}

			i += 1;
		}

		i
	}

	/// Return a non-mutable reference to the extended language subtags.
	#[inline]
	pub fn as_ref(&self) -> &LanguageExtension {
		let i = self.primary_len()+1;
		if i < self.p.language_end {
			unsafe {
				LanguageExtension::parse_unchecked(&self.buffer[i..])
			}
		} else {
			LanguageExtension::empty()
		}
	}

	/// Checks if no extended language subtags are defined.
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.as_ref().is_empty()
	}

	/// Checks if no more extended language subtags can be inserted. 
	#[inline]
	pub fn is_full(&self) -> bool {
		self.as_ref().is_full()
	}

	/// Checks if the given subtag is present.
	#[inline]
	pub fn contains<T: AsRef<[u8]> + ?Sized>(&self, subtag: &T) -> bool {
		self.as_ref().contains(subtag)
	}

	/// Insert a new subtag if it there is room for it and is not already present.
	/// 
	/// Return `true` if the subtag has been inserted,
	/// and `false` if the subtag was already present or
	/// if there already are 3 extended language subtags.
	#[inline]
	pub fn insert(&mut self, subtag: &ExtendedLangTag) -> bool {
		if !self.is_full() && !self.contains(subtag) {
			let i = self.p.language_end;
			crate::replace(self.buffer, i..i, subtag.as_ref());

			// a subtag separator.
			crate::replace(self.buffer, i..i, b"-");

			let new_end = self.p.language_end + 1 + subtag.len();
			let diff = new_end - self.p.language_end;
			self.p.script_end += diff;
			self.p.region_end += diff;
			self.p.variant_end += diff;
			self.p.extension_end += diff;
			self.p.privateuse_end += diff;
			self.p.language_end = new_end;

			true
		} else {
			false
		}
	}

	/// Remove all occurences of the given subtag.
	/// 
	/// Return `true` if the subtag was present and `false` otherwise.
	#[inline]
	pub fn remove<T: AsRef<[u8]> + ?Sized>(&mut self, subtag: &T) -> bool {
		let primary_len = self.primary_len();
		let mut i = primary_len+1; // current visited byte index.
		let mut subtag_offset = primary_len; // offset of the current subtag (including the `-` prefix).
		let mut removed = false; // did we remove some subtag?
		let mut new_end = self.p.language_end;

		while i < self.buffer.len() {
			if self.buffer[i] == b'-' {
				// if the current subtag matches the subtag to remove.
				if &self.buffer[(subtag_offset+1)..i] == subtag.as_ref() {
					let len = i-subtag_offset;
					crate::replace(self.buffer, subtag_offset..i, &[]);
					i -= len;
					new_end -= len;
					removed = true
				}

				subtag_offset = i;
			}

			i += 1
		}

		// if the subtag to remove is in last position.
		if &self.buffer[(subtag_offset+1)..i] == subtag.as_ref() {
			let len = i-subtag_offset;
			crate::replace(self.buffer, subtag_offset..i, &[]);
			new_end -= len;
			removed = true
		}

		if removed {
			let diff = self.p.language_end - new_end;
			self.p.script_end -= diff;
			self.p.region_end -= diff;
			self.p.variant_end -= diff;
			self.p.extension_end -= diff;
			self.p.privateuse_end -= diff;
			self.p.language_end = new_end;
		}

		removed
	}
}