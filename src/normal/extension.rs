use core::{fmt, hash::Hash};

use crate::utils;

#[derive(Debug, thiserror::Error)]
#[error("invalid extension identifier")]
pub struct InvalidSingleton<T>(pub T);

/// Extension identifier.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Singleton(u8);

impl Singleton {
	pub fn new(c: u8) -> Result<Self, InvalidSingleton<u8>> {
		if c != b'x' && c != b'X' && c.is_ascii_alphanumeric() {
			Ok(Self(c))
		} else {
			Err(InvalidSingleton(c))
		}
	}

	pub fn from_string(str: &str) -> Result<Self, InvalidSingleton<&str>> {
		if str.len() == 1 {
			str.chars()
				.next()
				.unwrap()
				.try_into()
				.map_err(|_| InvalidSingleton(str))
		} else {
			Err(InvalidSingleton(str))
		}
	}

	/// Convert into the underlying byte.
	#[inline]
	pub fn unwrap(self) -> u8 {
		self.0
	}
}

#[cfg(feature = "std")]
impl core::str::FromStr for Singleton {
	type Err = InvalidSingleton<String>;

	fn from_str(str: &str) -> Result<Self, InvalidSingleton<String>> {
		Self::from_string(str).map_err(|_| InvalidSingleton(str.to_owned()))
	}
}

impl TryFrom<u8> for Singleton {
	type Error = InvalidSingleton<u8>;

	#[inline]
	fn try_from(b: u8) -> Result<Singleton, InvalidSingleton<u8>> {
		Self::new(b)
	}
}

impl TryFrom<char> for Singleton {
	type Error = InvalidSingleton<char>;

	#[inline]
	fn try_from(c: char) -> Result<Singleton, InvalidSingleton<char>> {
		let codepoint = c as u32;

		if codepoint < u8::MAX as u32 {
			Self::new(codepoint as u8).map_err(|_| InvalidSingleton(c))
		} else {
			Err(InvalidSingleton(c))
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
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Extension)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(ExtensionBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct Extension(str);

impl Extension {
	pub fn singleton(&self) -> Singleton {
		Singleton(self.0.as_bytes()[0])
	}

	pub fn iter(&self) -> ExtensionIter<'_> {
		ExtensionIter::new(&self.0)
	}
}

impl PartialEq for Extension {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for Extension {}

impl PartialOrd for Extension {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Extension {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for Extension {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}

#[derive(Default)]
pub struct ExtensionIter<'a> {
	data: &'a str,
	offset: usize,
}

impl<'a> ExtensionIter<'a> {
	fn new(data: &'a str) -> Self {
		Self {
			data,
			offset: 2, // start after the singleton
		}
	}

	pub fn empty() -> Self {
		Self::default()
	}
}

impl<'a> Iterator for ExtensionIter<'a> {
	type Item = &'a ExtensionSubtag;

	fn next(&mut self) -> Option<Self::Item> {
		if self.offset < self.data.len() {
			let end = super::find_segment_end(self.data, self.offset);
			let item = unsafe { ExtensionSubtag::new_unchecked(&self.data[self.offset..end]) };
			self.offset = end + 1;
			Some(item)
		} else {
			None
		}
	}
}

/// List of extensions.
///
/// A list of language tag extension, separated by a `-` character.
/// Individual extensions are represented by the [`Extension`] type,
/// while extension subtags are represented by the [`ExtensionSubtag`]
/// type.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Extensions)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(ExtensionsBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct Extensions(str);

impl Extensions {
	pub const EMPTY: &Self = unsafe { Self::new_unchecked("") };

	pub fn get(&self, singleton: Singleton) -> Option<&Extension> {
		let bytes = self.0.as_bytes();
		let mut i = 0;
		let mut starting = true;
		while i + 1 < self.0.len() {
			if starting {
				if bytes[i] == singleton.0 && bytes[i + 1] == b'-' {
					let end = super::find_list_end(&self.0, i + 2, |_, segment| segment.len() > 1);

					return Some(unsafe { Extension::new_unchecked(&self.0[i..end]) });
				}

				starting = false
			} else if bytes[i] == b'-' {
				starting = true
			}

			i += 1
		}

		None
	}

	pub fn iter(&self) -> ExtensionsIter<'_> {
		ExtensionsIter::new(&self.0)
	}

	pub fn iter_extension(&self, singleton: Singleton) -> ExtensionIter<'_> {
		self.get(singleton).map(Extension::iter).unwrap_or_default()
	}
}

impl PartialEq for Extensions {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for Extensions {}

impl PartialOrd for Extensions {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Extensions {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for Extensions {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}

pub struct ExtensionsIter<'a> {
	data: &'a str,
	offset: usize,
}

impl<'a> ExtensionsIter<'a> {
	fn new(data: &'a str) -> Self {
		Self { data, offset: 0 }
	}
}

impl<'a> Iterator for ExtensionsIter<'a> {
	type Item = &'a Extension;

	fn next(&mut self) -> Option<Self::Item> {
		if self.offset < self.data.len() {
			let offset = self.offset + 2; // skip singleton;
			let end = super::find_list_end(self.data, offset, |_, segment| {
				ExtensionSubtag::new(segment).is_ok()
			});

			let item = unsafe { Extension::new_unchecked(&self.data[self.offset..end]) };

			self.offset = end + 1;
			Some(item)
		} else {
			None
		}
	}
}

/// Single extension subtag.
///
/// Extension subtag found in a language tag extension.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::ExtensionSubtag)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(ExtensionSubtagBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct ExtensionSubtag(str);

impl PartialEq for ExtensionSubtag {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for ExtensionSubtag {}

impl PartialOrd for ExtensionSubtag {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for ExtensionSubtag {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for ExtensionSubtag {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
