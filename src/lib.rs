//! This crate provides an implementation of *language tags* defined by
//! [RFC5646](https://tools.ietf.org/html/rfc5646) ([BCP47](https://tools.ietf.org/html/bcp47)).
//!
//! ## Usage
//!
//! You can easily parse new language from anything that provides a `[u8]` reference:
//! ```rust
//! extern crate langtag;
//! use langtag::LanguageTag;
//!
//! fn main() -> Result<(), langtag::Error> {
//!   let tag = LanguageTag::parse("fr-FR")?;
//!   assert_eq!(tag.language().unwrap().primary(), "fr");
//!   assert!(tag == "Fr-fr"); // comparison is case-insensitive.
//!   Ok(())
//! }
//! ```
//!
//! Note that [`LanguageTag::parse`] does *not* copy the data it is given,
//! but only borrows it.
//! You can create an owning `LanguageTag` instance by using
//! [`LanguageTagBuf::parse_copy`] to copy the data,
//! or simply [`LanguageTagBuf::new`] to move the data.
//!
//! Once parsed, you can explore every component of the language tag using the provided functions.
//!
//! ### Mutable language tags
//!
//! When the language tags owns its buffer through `Vec<u8>`,
//! it becomes possible to access the tag mutably to modify it.
//! ```rust
//! extern crate langtag;
//! use std::convert::TryInto;
//! use langtag::LangTag;
//!
//! fn main() -> Result<(), langtag::Error> {
//!   let mut tag = LangTag::parse_copy("fr-FR")?;
//!   tag.language_mut().set_primary("jp".try_into()?);
//!   tag.set_region(None);
//!   tag.extensions_mut().insert('f'.try_into()?, "bar".try_into()?);
//!   assert_eq!(tag, "jp-f-bar");
//!   Ok(())
//! }
//! ```
use std::{
	cmp::{Ord, Ordering, PartialOrd},
	convert::TryFrom,
	fmt,
	hash::{Hash, Hasher},
	ops::{Deref, Range},
};

macro_rules! component {
	($(#[ doc = $doc:tt ])* $parser:ident, $multi:expr, $id:ident, $err:ident) => {
		$(#[doc=$doc])*
		pub struct $id<T: ?Sized = [u8]> {
			pub(crate) data: T
		}

		impl<T: AsRef<[u8]>> $id<T> {
			/// Parse and use the given data.
			#[inline]
			pub fn new(t: T) -> Result<$id<T>, (Error, T)> {
				let bytes = t.as_ref();

				if ($multi || bytes.len() > 0) && crate::parse::$parser(bytes, 0) == bytes.len() {
					Ok($id {
						data: t
					})
				} else {
					Err((Error::$err, t))
				}
			}

			/// Use the given data as buffer without parsing it.
			///
			/// ## Safety
			/// The given data must be syntactically correct.
			#[inline]
			pub unsafe fn new_unchecked(t: T) -> $id<T> {
				$id {
					data: t
				}
			}
		}

		impl $id<[u8]> {
			/// Parse and borrow the given data.
			#[inline]
			pub fn parse<'a, T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> Result<&'a $id<[u8]>, Error> {
				let bytes = bytes.as_ref();

				if ($multi || bytes.len() > 0) && crate::parse::$parser(bytes, 0) == bytes.len() {
					Ok(unsafe {
						&*(bytes as *const [u8] as *const $id<[u8]>)
					})
				} else {
					Err(Error::$err)
				}
			}

			/// Borrow the given data without checking that it is syntactically correct.
			///
			/// ## Safety
			/// The data must be syntactically correct.
			#[inline]
			pub unsafe fn parse_unchecked<'a, T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> &'a $id<[u8]> {
				&*(bytes.as_ref() as *const [u8] as *const $id<[u8]>)
			}
		}

		impl $id<Vec<u8>> {
			/// Parse and copy the input data.
			#[inline]
			pub fn parse_copy<'a, T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> Result<$id<Vec<u8>>, Error> {
				let bytes = bytes.as_ref();
				let mut buffer = Vec::new();
				buffer.resize(bytes.len(), 0);
				buffer.copy_from_slice(bytes);
				$id::new(buffer).map_err(|(e, _)| e)
			}

			/// Copy the input data without checking its syntax correctness.
			///
			/// ## Safety
			/// The input data must be syntactically correct.
			#[inline]
			pub unsafe fn parse_copy_unchecked<'a, T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> $id<Vec<u8>> {
				let bytes = bytes.as_ref();
				let mut buffer = Vec::new();
				buffer.resize(bytes.len(), 0);
				buffer.copy_from_slice(bytes);
				$id::new_unchecked(buffer)
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> $id<T> {
			#[inline]
			/// Bytes length.
			pub fn len(&self) -> usize {
				self.as_bytes().len()
			}

			#[inline]
			pub fn is_empty(&self) -> bool {
				self.as_bytes().is_empty()
			}

			#[inline]
			/// Return a reference to the underlying data.
			pub fn as_bytes(&self) -> &[u8] {
				self.data.as_ref()
			}

			#[inline]
			/// Return a reference to the underlying data as a string.
			pub fn as_str(&self) -> &str {
				unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
			}
		}

		impl<'a> TryFrom<&'a str> for &'a $id<[u8]> {
			type Error = Error;

			#[inline]
			fn try_from(b: &'a str) -> Result<&'a $id<[u8]>, Error> {
				$id::parse(b.as_bytes())
			}
		}

		impl<'a> TryFrom<&'a [u8]> for &'a $id<[u8]> {
			type Error = Error;

			#[inline]
			fn try_from(b: &'a [u8]) -> Result<&'a $id<[u8]>, Error> {
				$id::parse(b)
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> AsRef<[u8]> for $id<T> {
			#[inline]
			fn as_ref(&self) -> &[u8] {
				self.as_bytes()
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> AsRef<str> for $id<T> {
			#[inline]
			fn as_ref(&self) -> &str {
				self.as_str()
			}
		}

		impl<T: ?Sized> Deref for $id<T> {
			type Target = T;

			#[inline]
			fn deref(&self) -> &T {
				&self.data
			}
		}

		impl<T: AsRef<[u8]> + ?Sized, U: AsRef<[u8]> + ?Sized> PartialOrd<$id<U>> for $id<T> {
			#[inline]
			fn partial_cmp(&self, other: &$id<U>) -> Option<Ordering> {
				Some(crate::case_insensitive_cmp(self.as_bytes(), other.as_bytes()))
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> Ord for $id<T> {
			#[inline]
			fn cmp(&self, other: &$id<T>) -> Ordering {
				crate::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
			}
		}

		impl<T: AsRef<[u8]> + ?Sized, U: AsRef<[u8]> + ?Sized> PartialEq<$id<U>> for $id<T> {
			#[inline]
			fn eq(&self, other: &$id<U>) -> bool {
				crate::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> PartialEq<[u8]> for $id<T> {
			#[inline]
			fn eq(&self, other: &[u8]) -> bool {
				crate::case_insensitive_eq(self.as_bytes(), other)
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> PartialEq<str> for $id<T> {
			#[inline]
			fn eq(&self, other: &str) -> bool {
				crate::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> PartialEq<$id<T>> for [u8] {
			#[inline]
			fn eq(&self, other: &$id<T>) -> bool {
				crate::case_insensitive_eq(self, other.as_bytes())
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> PartialEq<$id<T>> for str {
			#[inline]
			fn eq(&self, other: &$id<T>) -> bool {
				crate::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> Eq for $id<T> {}

		impl<T: AsRef<[u8]> + ?Sized> Hash for $id<T> {
			#[inline]
			fn hash<H: Hasher>(&self, h: &mut H) {
				crate::case_insensitive_hash(self.as_bytes(), h)
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> fmt::Display for $id<T> {
			#[inline]
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				fmt::Display::fmt(self.as_str(), f)
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> fmt::Debug for $id<T> {
			#[inline]
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				fmt::Debug::fmt(self.as_str(), f)
			}
		}
	};
}

macro_rules! iterator {
	($(#[ doc = $doc:tt ])* $collection:ident, $id:ident, $item:ident, $offset:literal) => {
		$(#[doc = $doc])*
		pub struct $id<'a> {
			bytes: &'a [u8],
			i: usize
		}

		impl<'a> Iterator for $id<'a> {
			type Item = &'a $item;

			#[inline]
			fn next(&mut self) -> Option<&'a $item> {
				if self.i < self.bytes.len() {
					if self.bytes[self.i] == b'-' {
						self.i += 1;
					}

					let offset = self.i;

					while self.i < self.bytes.len() && self.bytes[self.i] != b'-' {
						self.i += 1;
					}

					unsafe {
						Some($item::parse_unchecked(&self.bytes[offset..self.i]))
					}
				} else {
					None
				}
			}
		}

		impl $collection {
			/// Empty list (the empty string).
			#[inline]
			pub fn empty() -> &'static $collection {
				unsafe {
					$collection::parse_unchecked(b"")
				}
			}

			/// Returns the first subtag of the list (if any).
			#[inline]
			pub fn first(&self) -> Option<&$item> {
				if self.is_empty() {
					None
				} else {
					let bytes = self.as_bytes();
					let mut i = $offset;

					while i < bytes.len() && bytes[i] != b'-' {
						i += 1
					}

					unsafe {
						Some($item::parse_unchecked(&bytes[..i]))
					}
				}
			}

			/// Returns the last subtag of the list (if any).
			#[inline]
			pub fn last(&self) -> Option<&$item> {
				if self.is_empty() {
					None
				} else {
					let bytes = self.as_bytes();
					let mut i = bytes.len()-1;

					while i > $offset+1 && bytes[i-1] != b'-' {
						i -= 1
					}

					unsafe {
						Some($item::parse_unchecked(&bytes[i..]))
					}
				}
			}

			/// Iterate through the subtags of the list.
			#[inline]
			pub fn iter(&self) -> $id {
				$id {
					bytes: self.as_bytes(),
					i: $offset
				}
			}

			/// Checks if the given subtag is included the list.
			#[inline]
			pub fn contains<T: AsRef<[u8]> + ?Sized>(&self, subtag: &T) -> bool {
				for st in self.iter() {
					if st == subtag.as_ref() {
						return true
					}
				}

				false
			}
		}

		impl<'a> IntoIterator for &'a $collection {
			type Item = &'a $item;
			type IntoIter = $id<'a>;

			#[inline]
			fn into_iter(self) -> $id<'a> {
				$id {
					bytes: self.as_bytes(),
					i: $offset
				}
			}
		}
	};
}

mod error;
mod extension;
mod grandfathered;
mod langtag;
mod language;
mod parse;
mod privateuse;
mod variant;

pub use self::langtag::*;
pub use error::*;
pub use extension::*;
pub use grandfathered::*;
pub use language::*;
pub use privateuse::*;
pub use variant::*;

/// Language tag with borrowed data.
pub enum LanguageTag<'a, T: ?Sized = [u8]> {
	/// Normal language tag.
	Normal(LangTag<&'a T>),

	/// Private use tag.
	PrivateUse(&'a PrivateUseTag<T>),

	/// Grandfathered tag.
	Grandfathered(GrandfatheredTag),
}

/// Language tag with owned data.
#[derive(Clone, Copy)]
pub enum LanguageTagBuf<T = Vec<u8>> {
	/// Normal language tag.
	Normal(LangTag<T>),

	/// Private use tag.
	PrivateUse(PrivateUseTag<T>),

	/// Grandfathered tag.
	Grandfathered(GrandfatheredTag),
}

impl<T: AsRef<[u8]>> LanguageTagBuf<T> {
	/// Create a new language tag parsing and using the given data.
	#[inline]
	pub fn new(t: T) -> Result<LanguageTagBuf<T>, (Error, T)> {
		match GrandfatheredTag::new(t) {
			Ok(tag) => Ok(LanguageTagBuf::Grandfathered(tag)),
			Err(t) => match PrivateUseTag::new(t) {
				Ok(tag) => Ok(LanguageTagBuf::PrivateUse(tag)),
				Err(t) => Ok(LanguageTagBuf::Normal(LangTag::new(t)?)),
			},
		}
	}

	/// Returns a [`LanguageTag`] referencing this tag.
	#[inline]
	pub fn as_ref(&self) -> LanguageTag {
		match self {
			LanguageTagBuf::Normal(tag) => unsafe {
				LanguageTag::Normal(LangTag::from_raw_parts(tag.as_bytes(), tag.parsing_data()))
			},
			LanguageTagBuf::PrivateUse(tag) => unsafe {
				LanguageTag::PrivateUse(PrivateUseTag::parse_unchecked(tag))
			},
			LanguageTagBuf::Grandfathered(tag) => LanguageTag::Grandfathered(*tag),
		}
	}
}

impl LanguageTagBuf {
	/// reate a new language tag owning its data by parsing and copying the given data.
	#[inline]
	pub fn parse_copy<T: AsRef<[u8]> + ?Sized>(t: &T) -> Result<LanguageTagBuf, Error> {
		let bytes = t.as_ref();
		let mut buffer = Vec::new();
		buffer.resize(bytes.len(), 0);
		buffer.copy_from_slice(bytes);
		Self::new(buffer).map_err(|(e, _)| e)
	}
}

macro_rules! language_tag_impl {
	() => {
		/// Returns the bytes representation of the language tag.
		#[inline]
		pub fn as_bytes(&self) -> &[u8] {
			match self {
				Self::Normal(tag) => tag.as_bytes(),
				Self::PrivateUse(tag) => tag.as_bytes(),
				Self::Grandfathered(tag) => tag.as_bytes(),
			}
		}

		/// Returns the string representation of the language tag.
		#[inline]
		pub fn as_str(&self) -> &str {
			match self {
				Self::Normal(tag) => tag.as_str(),
				Self::PrivateUse(tag) => tag.as_str(),
				Self::Grandfathered(tag) => tag.as_str(),
			}
		}

		/// Checks if this is a normal language tag.
		#[inline]
		pub fn is_normal(&self) -> bool {
			match self {
				Self::Normal(_) => true,
				_ => false,
			}
		}

		/// Checks if this is a private use tag.
		#[inline]
		pub fn is_private_use(&self) -> bool {
			match self {
				Self::PrivateUse(_) => true,
				_ => false,
			}
		}

		/// Checks if this is a grandfathered tag.
		#[inline]
		pub fn is_grandfathered(&self) -> bool {
			match self {
				Self::Grandfathered(_) => true,
				_ => false,
			}
		}

		/// Get the language subtags, if any.
		///
		/// Only normal language tags and regular grandfathered tags have language subtags.
		#[inline]
		pub fn language(&self) -> Option<&Language> {
			match self {
				Self::Normal(tag) => Some(tag.language()),
				Self::PrivateUse(_) => None,
				Self::Grandfathered(tag) => tag.language(),
			}
		}

		/// Get the script subtag, if any.
		#[inline]
		pub fn script(&self) -> Option<&Script> {
			match self {
				Self::Normal(tag) => tag.script(),
				Self::PrivateUse(_) => None,
				Self::Grandfathered(_) => None,
			}
		}

		/// Get the region subtag, if any.
		#[inline]
		pub fn region(&self) -> Option<&Region> {
			match self {
				Self::Normal(tag) => tag.region(),
				Self::PrivateUse(_) => None,
				Self::Grandfathered(_) => None,
			}
		}

		/// Get the variant subtags.
		#[inline]
		pub fn variants(&self) -> &Variants {
			match self {
				Self::Normal(tag) => tag.variants(),
				Self::PrivateUse(_) => Variants::empty(),
				Self::Grandfathered(_) => Variants::empty(),
			}
		}

		/// Get the extension subtags.
		#[inline]
		pub fn extensions(&self) -> &Extensions {
			match self {
				Self::Normal(tag) => tag.extensions(),
				Self::PrivateUse(_) => Extensions::empty(),
				Self::Grandfathered(_) => Extensions::empty(),
			}
		}

		/// Get the private use subtags.
		#[inline]
		pub fn private_use_subtags(&self) -> &PrivateUseSubtags {
			match self {
				Self::Normal(tag) => tag.private_use_subtags(),
				Self::PrivateUse(tag) => (*tag).subtags(),
				Self::Grandfathered(_) => PrivateUseSubtags::empty(),
			}
		}
	};
}

impl<'a, T: AsRef<[u8]> + ?Sized> LanguageTag<'a, T> {
	language_tag_impl!();
}

impl<'a> LanguageTag<'a> {
	/// Create a new language tag by parsing and borrowing the given data.
	#[inline]
	pub fn parse<T: AsRef<[u8]> + ?Sized>(t: &'a T) -> Result<LanguageTag<'a>, Error> {
		match GrandfatheredTag::new(t) {
			Ok(tag) => Ok(LanguageTag::Grandfathered(tag)),
			Err(_) => match PrivateUseTag::parse(t) {
				Ok(tag) => Ok(LanguageTag::PrivateUse(tag)),
				Err(_) => Ok(LanguageTag::Normal(LangTag::parse(t)?)),
			},
		}
	}

	#[inline]
	pub fn cloned(&self) -> LanguageTagBuf {
		match self {
			LanguageTag::Normal(tag) => unsafe {
				let mut buffer = Vec::new();
				buffer.extend_from_slice(tag.as_bytes());
				LanguageTagBuf::Normal(LangTag::from_raw_parts(buffer, tag.parsing_data()))
			},
			LanguageTag::PrivateUse(tag) => unsafe {
				let mut buffer = Vec::new();
				buffer.extend_from_slice(tag.as_bytes());
				LanguageTagBuf::PrivateUse(PrivateUseTag::new_unchecked(buffer))
			},
			LanguageTag::Grandfathered(tag) => LanguageTagBuf::Grandfathered(*tag),
		}
	}
}

impl<'a, T: ?Sized> Clone for LanguageTag<'a, T> {
	fn clone(&self) -> LanguageTag<'a, T> {
		match self {
			LanguageTag::Normal(tag) => LanguageTag::Normal(*tag),
			LanguageTag::PrivateUse(tag) => LanguageTag::PrivateUse(*tag),
			LanguageTag::Grandfathered(tag) => LanguageTag::Grandfathered(*tag),
		}
	}
}

impl<'a, T: ?Sized> Copy for LanguageTag<'a, T> {}

impl<'a, T: AsRef<[u8]> + ?Sized> AsRef<[u8]> for LanguageTag<'a, T> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<'a, T: AsRef<[u8]> + ?Sized> AsRef<str> for LanguageTag<'a, T> {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<'a, T: AsRef<[u8]> + ?Sized, U: AsRef<[u8]> + ?Sized> PartialEq<U> for LanguageTag<'a, T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		case_insensitive_eq(self.as_bytes(), other.as_ref())
	}
}

impl<'a, T: AsRef<[u8]> + ?Sized> Eq for LanguageTag<'a, T> {}

impl<'a, T: AsRef<[u8]> + ?Sized> Hash for LanguageTag<'a, T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		case_insensitive_hash(self.as_bytes(), h)
	}
}

impl<'a, T: AsRef<[u8]> + ?Sized, U: AsRef<[u8]>> PartialOrd<U> for LanguageTag<'a, T> {
	fn partial_cmp(&self, other: &U) -> Option<Ordering> {
		Some(case_insensitive_cmp(self.as_bytes(), other.as_ref()))
	}
}

impl<'a, T: AsRef<[u8]> + ?Sized> Ord for LanguageTag<'a, T> {
	fn cmp(&self, other: &Self) -> Ordering {
		case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Display for LanguageTag<'a, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<'a, T: AsRef<[u8]> + ?Sized> fmt::Debug for LanguageTag<'a, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]>> LanguageTagBuf<T> {
	language_tag_impl!();
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for LanguageTagBuf<T> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<T: AsRef<[u8]>> AsRef<str> for LanguageTagBuf<T> {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]> + ?Sized> PartialEq<U> for LanguageTagBuf<T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		case_insensitive_eq(self.as_bytes(), other.as_ref())
	}
}

impl<T: AsRef<[u8]>> Eq for LanguageTagBuf<T> {}

impl<T: AsRef<[u8]>> Hash for LanguageTagBuf<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		case_insensitive_hash(self.as_bytes(), h)
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]>> PartialOrd<U> for LanguageTagBuf<T> {
	fn partial_cmp(&self, other: &U) -> Option<Ordering> {
		Some(case_insensitive_cmp(self.as_bytes(), other.as_ref()))
	}
}

impl<T: AsRef<[u8]>> Ord for LanguageTagBuf<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl<T: AsRef<[u8]>> fmt::Display for LanguageTagBuf<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]>> fmt::Debug for LanguageTagBuf<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}

/// Normal language tag owning its data using [`Vec<u8>`].
///
/// Such language tag provides additional functions to modify the tag and its subtags.
pub type LangTagBuf = LangTag<Vec<u8>>;

/// Private use tag owning its data using [`Vec<u8>`].
///
/// Such private use tag provides additional functions to modify its subtags.
pub type PrivateUseTagBuf = PrivateUseTag<Vec<u8>>;

#[inline]
pub(crate) fn into_smallcase(c: u8) -> u8 {
	if (b'A'..=b'Z').contains(&c) {
		c + 0x20
	} else {
		c
	}
}

#[inline]
pub(crate) fn case_insensitive_eq(a: &[u8], b: &[u8]) -> bool {
	if a.len() == b.len() {
		for i in 0..a.len() {
			if into_smallcase(a[i]) != into_smallcase(b[i]) {
				return false;
			}
		}

		true
	} else {
		false
	}
}

#[inline]
pub(crate) fn case_insensitive_hash<H: Hasher>(bytes: &[u8], hasher: &mut H) {
	for b in bytes {
		into_smallcase(*b).hash(hasher)
	}
}

#[inline]
pub(crate) fn case_insensitive_cmp(a: &[u8], b: &[u8]) -> Ordering {
	let mut i = 0;

	loop {
		if a.len() <= i {
			if b.len() <= i {
				return Ordering::Equal;
			}

			return Ordering::Greater;
		} else if b.len() <= i {
			return Ordering::Less;
		} else {
			match into_smallcase(a[i]).cmp(&into_smallcase(b[i])) {
				Ordering::Equal => i += 1,
				ord => return ord,
			}
		}
	}
}

component! {
	/// Script subtag.
	///
	/// Script subtags are used to indicate the script or writing system
	/// variations that distinguish the written forms of a language or its
	/// dialects.
	script, false, Script, InvalidScript
}

component! {
	/// Region subtag.
	///
	/// Region subtags are used to indicate linguistic variations associated
	/// with or appropriate to a specific country, territory, or region.
	/// Typically, a region subtag is used to indicate variations such as
	/// regional dialects or usage, or region-specific spelling conventions.
	/// It can also be used to indicate that content is expressed in a way
	/// that is appropriate for use throughout a region, for instance,
	/// Spanish content tailored to be useful throughout Latin America.
	region, false, Region, InvalidRegion
}

/// Replacement function.
///
/// Replace the given `range` of the input `buffer` with the given `content`.
/// This function is used in many places to replace parts of langtag buffer data.
pub(crate) fn replace(buffer: &mut Vec<u8>, range: Range<usize>, content: &[u8]) {
	let range_len = range.end - range.start;

	// move the content around.
	if range_len != content.len() {
		let tail_len = buffer.len() - range.end; // the length of the content in the buffer after [range].
		let new_end = range.start + content.len();

		if range_len > content.len() {
			// shrink
			for i in 0..tail_len {
				buffer[new_end + i] = buffer[range.end + i];
			}

			buffer.resize(new_end + tail_len, 0);
		} else {
			// grow
			let tail_len = buffer.len() - range.end;

			buffer.resize(new_end + tail_len, 0);

			for i in 0..tail_len {
				buffer[new_end + tail_len - i - 1] = buffer[range.end + tail_len - i - 1];
			}
		}
	}

	// actually replace the content.
	for i in 0..content.len() {
		buffer[range.start + i] = content[i]
	}
}
