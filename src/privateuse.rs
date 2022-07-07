use crate::Error;
use std::{
	cmp::Ordering,
	convert::TryFrom,
	fmt,
	hash::{Hash, Hasher},
	ops::Deref,
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

iterator! {
	/// Private use subtags iterator.
	PrivateUseSubtags, PrivateUseSubtagsIter, PrivateUseSubtag, 2
}

/// Mutable reference to private use subtags.
pub struct PrivateUseSubtagsMut<'a> {
	/// Language tag buffer.
	pub(crate) buffer: &'a mut Vec<u8>,

	/// Offset of the private use subtags in the buffer (including the preceding `-` separator).
	///
	/// The private use component is assumed to span to the buffer's end.
	pub(crate) offset: usize,
}

impl<'a> PrivateUseSubtagsMut<'a> {
	/// Return the component offset.
	///
	/// If the `offset` field is not 0, the it includes the preceding `-` separator so we must add 1.
	#[inline]
	fn component_offset(&self) -> usize {
		if self.offset > 0 {
			self.offset + 1
		} else {
			self.offset
		}
	}

	/// Returns a non-mutable reference to the private use subtags.
	#[inline]
	pub fn as_private_use_subtags(&self) -> &PrivateUseSubtags {
		let i = self.component_offset();

		if i < self.buffer.len() {
			unsafe { PrivateUseSubtags::parse_unchecked(&self.buffer[i..]) }
		} else {
			PrivateUseSubtags::empty()
		}
	}

	/// Checks if the subtag list is empty.
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
		let mut i = self.offset + 3; // current visited byte index.
		let mut subtag_offset = self.offset + 2; // offset of the current subtag (including the `-` prefix).
		let mut removed = false; // did we remove some subtag?

		while i < self.buffer.len() {
			if self.buffer[i] == b'-' {
				// if the current subtag matches the subtag to remove.
				if &self.buffer[(subtag_offset + 1)..i] == subtag.as_ref() {
					let len = i - subtag_offset;
					crate::replace(self.buffer, subtag_offset..i, &[]);
					i -= len;
					removed = true
				}

				subtag_offset = i;
			}

			i += 1
		}

		// if the subtag to remove is in last position.
		if &self.buffer[(subtag_offset + 1)..i] == subtag.as_ref() {
			crate::replace(self.buffer, subtag_offset..i, &[]);
			removed = true
		}

		// if there are no subtags left, remove the `x-` prefix.
		if self.buffer.len() - self.component_offset() == 1 {
			if self.offset > 0 {
				// if the offset if not 0, also remove the preceding `-` separator.
				crate::replace(self.buffer, self.offset..(self.offset + 2), &[]);
			} else {
				crate::replace(self.buffer, self.offset..(self.offset + 1), &[]);
			}
		}

		removed
	}
}

impl<'a> AsRef<PrivateUseSubtags> for PrivateUseSubtagsMut<'a> {
	/// Returns a non-mutable reference to the private use subtags.
	#[inline]
	fn as_ref(&self) -> &PrivateUseSubtags {
		self.as_private_use_subtags()
	}
}

/// Private use tag.
#[derive(Clone, Copy)]
pub struct PrivateUseTag<T: ?Sized = [u8]> {
	data: T,
}

impl<T: AsRef<[u8]>> PrivateUseTag<T> {
	/// Parse and use the given data as a private use tag.
	#[inline]
	pub fn new(t: T) -> Result<PrivateUseTag<T>, T> {
		let bytes = t.as_ref();
		if !bytes.is_empty() && crate::parse::privateuse(bytes, 0) == bytes.len() {
			Ok(PrivateUseTag { data: t })
		} else {
			Err(t)
		}
	}

	/// Use the given data as a private use tag without checking it.
	///
	/// ## Safety
	/// The given data must be a valid private use tag.
	#[inline]
	pub unsafe fn new_unchecked(t: T) -> PrivateUseTag<T> {
		PrivateUseTag { data: t }
	}
}

impl PrivateUseTag {
	/// Parse and borrow the given data as a private use tag.
	#[inline]
	pub fn parse<'a, T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<&'a PrivateUseTag, Error> {
		let bytes = bytes.as_ref();
		if !bytes.is_empty() && crate::parse::privateuse(bytes, 0) == bytes.len() {
			Ok(unsafe { &*(bytes as *const [u8] as *const PrivateUseTag) })
		} else {
			Err(Error::InvalidPrivateUseSubtags)
		}
	}

	/// Borrow the given data as a private use tag without checking it.
	///
	/// ## Safety
	/// The given data must be a valid private use tag.
	#[inline]
	pub unsafe fn parse_unchecked<'a, T: AsRef<[u8]>>(bytes: &T) -> &'a PrivateUseTag {
		let bytes = bytes.as_ref();
		&*(bytes as *const [u8] as *const PrivateUseTag)
	}
}

impl<T: AsRef<[u8]> + ?Sized> PrivateUseTag<T> {
	/// Returns the bytes representation of this private use tag.
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.data.as_ref()
	}

	/// Returns the string representation of this private use tag.
	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}

	/// Iterate through the subtags.
	#[inline]
	pub fn subtags(&self) -> &PrivateUseSubtags {
		unsafe { PrivateUseSubtags::parse_unchecked(self.as_bytes()) }
	}
}

impl<T: AsMut<Vec<u8>>> PrivateUseTag<T> {
	/// Modify the subtags.
	#[inline]
	pub fn subtags_mut(&mut self) -> PrivateUseSubtagsMut {
		PrivateUseSubtagsMut {
			buffer: self.data.as_mut(),
			offset: 0,
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
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]> + ?Sized> fmt::Debug for PrivateUseTag<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}
