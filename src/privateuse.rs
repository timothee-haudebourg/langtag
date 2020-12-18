use std::{
	fmt,
	hash::{
		Hash,
		Hasher
	},
	convert::TryFrom,
	ops::Deref,
	cmp::Ordering
};
use crate::{
	Error,
	component,
	iterator
};

component! {
	/// List of private use subtags.
	/// 
	/// Private use subtags component of a language tag.
	/// If not empty, it is composed of the prefix `x-` followed
	/// by a list of [`PrivateUseSubtag`] separated by the `-` character.
	privateuse, false, PrivateUseSubtags, InvalidPrivateUseSubtags
}

component! {
	/// Single private use subtag.
	/// 
	/// Private use subtags are used to indicate distinctions in language
	/// that are important in a given context by private agreement.
	privateuse_subtag, false, PrivateUseSubtag, InvalidPrivateUseSubtag
}

iterator!(PrivateUseSubtags, PrivateUseSubtagsIter, PrivateUseSubtag, 2);

pub struct PrivateUseSubtagsMut<'a> {
	/// Language tag buffer.
	pub(crate) buffer: &'a mut Vec<u8>,

	/// Offset of the private use subtags in the buffer (including the preceding `-` separator).
	/// 
	/// The private use component is assumed to span to the buffer's end.
	pub(crate) offset: usize
}

impl<'a> PrivateUseSubtagsMut<'a> {
	/// Return the component offset.
	/// 
	/// If the `offset` field is not 0, the it includes the preceding `-` separator so we must add 1.
	#[inline]
	fn component_offset(&self) -> usize {
		if self.offset > 0 {
			self.offset+1
		} else {
			self.offset
		}
	}

	#[inline]
	pub fn as_ref(&self) -> &PrivateUseSubtags {
		let i = self.component_offset();

		if i < self.buffer.len() {
			unsafe {
				PrivateUseSubtags::parse_unchecked(&self.buffer[i..])
			}
		} else {
			PrivateUseSubtags::empty()
		}
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.as_ref().is_empty()
	}

	/// Checks if the given subtag is present.
	#[inline]
	pub fn contains<T: AsRef<[u8]> + ?Sized>(&self, subtag: &T) -> bool {
		self.as_ref().contains(subtag)
	}

	/// Insert a new subtag if it is not already present.
	/// 
	/// Return `true` if the subtag has been inserted,
	/// and `false` if the subtag was already present.
	#[inline]
	pub fn insert(&mut self, subtag: &PrivateUseSubtag) -> bool {
		if !self.contains(subtag) {
			let is_empty = self.is_empty();
			let i = self.buffer.len();
			crate::replace(self.buffer, i..i, subtag.as_ref());

			// a subtag separator.
			if is_empty {
				// if the list is empty, add the `x-` prefix.
				if self.offset > 0 {
					// if the offset is not 0, then we must also add the preceding `-` separator.
					crate::replace(self.buffer, self.offset..self.offset, b"-x-");
				} else {
					crate::replace(self.buffer, self.offset..self.offset, b"x-");
				}
			} else {
				crate::replace(self.buffer, i..i, b"-");
			}

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
		let mut i = self.offset+3; // current visited byte index.
		let mut subtag_offset = self.offset+2; // offset of the current subtag (including the `-` prefix).
		let mut removed = false; // did we remove some subtag?

		while i < self.buffer.len() {
			if self.buffer[i] == b'-' {
				// if the current subtag matches the subtag to remove.
				if &self.buffer[(subtag_offset+1)..i] == subtag.as_ref() {
					let len = i-subtag_offset;
					crate::replace(self.buffer, subtag_offset..i, &[]);
					i -= len;
					removed = true
				}

				subtag_offset = i;
			}

			i += 1
		}

		// if the subtag to remove is in last position.
		if &self.buffer[(subtag_offset+1)..i] == subtag.as_ref() {
			crate::replace(self.buffer, subtag_offset..i, &[]);
			removed = true
		}

		// if there are no subtags left, remove the `x-` prefix.
		if self.buffer.len()-self.component_offset() == 1 {
			if self.offset > 0 {
				// if the offset if not 0, also remove the preceding `-` separator.
				crate::replace(self.buffer, self.offset..(self.offset+2), &[]);
			} else {
				crate::replace(self.buffer, self.offset..(self.offset+1), &[]);
			}
		}

		removed
	}
}

pub struct PrivateUseTag<T: ?Sized = [u8]> {
	data: T
}

impl<T: AsRef<[u8]>> PrivateUseTag<T> {
	#[inline]
	pub fn new(t: T) -> Result<PrivateUseTag<T>, T> {
		let bytes = t.as_ref();
		if bytes.len() > 0 && crate::parse::privateuse(bytes, 0) == bytes.len() {
			Ok(PrivateUseTag {
				data: t
			})
		} else {
			Err(t)
		}
	}
}

impl PrivateUseTag {
	pub fn parse<'a, T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<&'a PrivateUseTag, Error> {
		let bytes = bytes.as_ref();
		if bytes.len() > 0 && crate::parse::privateuse(bytes, 0) == bytes.len() {
			Ok(unsafe {
				&*(bytes as *const [u8] as *const PrivateUseTag)
			})
		} else {
			Err(Error::InvalidPrivateUseSubtags)
		}
	}
}

impl<T: AsRef<[u8]> + ?Sized> PrivateUseTag<T> {
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.data.as_ref()
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe {
			std::str::from_utf8_unchecked(self.as_bytes())
		}
	}

	#[inline]
	pub fn subtags(&self) -> &PrivateUseSubtags {
		unsafe {
			PrivateUseSubtags::parse_unchecked(self.as_bytes())
		}
	}
}

impl<T: AsMut<Vec<u8>>> PrivateUseTag<T> {
	#[inline]
	pub fn subtags_mut(&mut self) -> PrivateUseSubtagsMut {
		PrivateUseSubtagsMut {
			buffer: self.data.as_mut(),
			offset: 0
		}
	}
}

impl<T: AsRef<[u8]> + ?Sized> AsRef<[u8]> for PrivateUseTag<T> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<T: AsRef<[u8]> + ?Sized, U: AsRef<[u8]> + ?Sized> PartialEq<U> for PrivateUseTag<T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		crate::case_insensitive_eq(self.data.as_ref(), other.as_ref())
	}
}

impl<T: AsRef<[u8]> + ?Sized> Eq for PrivateUseTag<T> {}

impl<T: AsRef<[u8]> + ?Sized> Hash for PrivateUseTag<T> {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		crate::case_insensitive_hash(self.data.as_ref(), h)
	}
}

impl<T: AsRef<[u8]> + ?Sized> fmt::Display for PrivateUseTag<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]> + ?Sized> fmt::Debug for PrivateUseTag<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}