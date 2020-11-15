use std::{
	fmt,
	hash::{
		Hash,
		Hasher
	},
	convert::TryFrom,
	cmp::{
		PartialOrd,
		Ord,
		Ordering
	},
	ops::Deref
};

mod error;
mod parse;
pub mod raw;
mod grandfathered;

pub use error::*;
pub use grandfathered::*;

impl LanguageTagBuf {
	#[inline]
	pub fn parse<T: AsRef<[u8]> + ?Sized>(t: &T) -> Result<LanguageTagBuf, parse::Error> {
		let mut buffer = Vec::new();
		buffer.copy_from_slice(t.as_ref());
		Self::new(buffer)
	}
}

impl<'a> LanguageTag<'a> {
	#[inline]
	pub fn parse<T: AsRef<[u8]> + ?Sized>(t: &'a T) -> Result<LanguageTag<'a>, parse::Error> {
		Self::new(t.as_ref())
	}
}

pub type LangTag<'a> = raw::LangTag<&'a [u8]>;
pub type LangTagBuf = raw::LangTag<Vec<u8>>;

pub type PrivateUseTag<'a> = raw::PrivateUseTag<&'a [u8]>;
pub type PrivateUseTagBuf = raw::PrivateUseTag<Vec<u8>>;

pub type LanguageTag<'a> = raw::LanguageTag<&'a [u8]>;
pub type LanguageTagBuf = raw::LanguageTag<Vec<u8>>;

#[inline]
pub(crate) fn into_smallcase(c: u8) -> u8 {
	if c >= b'A' && c <= b'Z' {
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
				return false
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
				return Ordering::Equal
			}

			return Ordering::Greater
		} else if b.len() <= i {
			return Ordering::Less
		} else {
			match into_smallcase(a[i]).cmp(&into_smallcase(b[i])) {
				Ordering::Equal => {
					i += 1
				},
				ord => return ord
			}
		}
	}
}

macro_rules! component {
	($parser:ident, $multi:expr, $doc:tt, $id:ident, $err:ident) => {
		#[doc=$doc]
		pub struct $id {
			data: [u8]
		}

		impl $id {
			#[inline]
			pub fn new<B: AsRef<[u8]> + ?Sized>(bytes: &B) -> Result<&$id, Error> {
				let bytes = bytes.as_ref();
				
				if ($multi || bytes.len() > 0) && crate::parse::$parser(bytes.as_ref(), 0) == bytes.len() {
					Ok(unsafe { Self::new_unchecked(bytes) })
				} else {
					Err(Error::$err)
				}
			}

			#[inline]
			pub unsafe fn new_unchecked<B: AsRef<[u8]> + ?Sized>(bytes: &B) -> &$id {
				&*(bytes.as_ref() as *const [u8] as *const $id)
			}

			#[inline]
			pub fn as_bytes(&self) -> &[u8] {
				&self.data
			}

			#[inline]
			pub fn as_str(&self) -> &str {
				unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
			}
		}

		impl<'a> TryFrom<&'a str> for &'a $id {
			type Error = Error;

			#[inline]
			fn try_from(b: &'a str) -> Result<&'a $id, Error> {
				$id::new(b)
			}
		}

		impl<'a> TryFrom<&'a [u8]> for &'a $id {
			type Error = Error;

			#[inline]
			fn try_from(b: &'a [u8]) -> Result<&'a $id, Error> {
				$id::new(b)
			}
		}

		impl AsRef<[u8]> for $id {
			#[inline]
			fn as_ref(&self) -> &[u8] {
				self.as_bytes()
			}
		}

		impl AsRef<str> for $id {
			#[inline]
			fn as_ref(&self) -> &str {
				self.as_str()
			}
		}

		impl Deref for $id {
			type Target = [u8];

			#[inline]
			fn deref(&self) -> &[u8] {
				self.as_bytes()
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> PartialOrd<T> for $id {
			#[inline]
			fn partial_cmp(&self, other: &T) -> Option<Ordering> {
				Some(crate::case_insensitive_cmp(self.as_bytes(), other.as_ref()))
			}
		}

		impl Ord for $id {
			#[inline]
			fn cmp(&self, other: &$id) -> Ordering {
				crate::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> PartialEq<T> for $id {
			#[inline]
			fn eq(&self, other: &T) -> bool {
				crate::case_insensitive_eq(self.as_bytes(), other.as_ref())
			}
		}

		impl PartialEq<$id> for [u8] {
			#[inline]
			fn eq(&self, other: &$id) -> bool {
				crate::case_insensitive_eq(self, other.as_bytes())
			}
		}

		impl PartialEq<$id> for str {
			#[inline]
			fn eq(&self, other: &$id) -> bool {
				crate::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}

		impl Eq for $id {}

		impl Hash for $id {
			#[inline]
			fn hash<H: Hasher>(&self, h: &mut H) {
				crate::case_insensitive_hash(&self.data, h)
			}
		}

		impl fmt::Display for $id {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				fmt::Display::fmt(self.as_str(), f)
			}
		}

		impl fmt::Debug for $id {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				fmt::Debug::fmt(self.as_str(), f)
			}
		}
	};
}

component! {
	language, false,
	"Primary and extended language subtags."
	, Language, InvalidLangage
}

component!(primary_language, false, "Primary language subtag.", PrimaryLanguage, InvalidPrimaryLangage);
component!(extlang, false, "Extended language subtags.", LanguageExtension, InvalidLangageExtension);
component!(extlang_tag, false, "Single extended language subtag.", ExtendedLangTag, InvalidExtendedLangTag);
component!(script, false, "Script subtag.", Script, InvalidScript);
component!(region, false, "Region subtag.", Region, InvalidRegion);
component!(variant, false, "Single variant subtag.", Variant, InvalidVariant);
component!(variants, true, "Variant subtags.", Variants, InvalidVariants);
component!(extension, false, "Single extension and its subtags.", Extension, InvalidExtension);
component!(extension_subtag, false, "Single extension subtag.", ExtensionSubtag, InvalidExtensionSubtag);
component!(extensions, true, "Extension subtags.", Extensions, InvalidExtensions);
component!(privateuse, false, "Private use subtags.", PrivateUseSubtags, InvalidPrivateUseSubtags);
component!(privateuse_subtag, false, "Single private use subtag.", PrivateUseSubtag, InvalidPrivateUseSubtag);

impl Language {
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
	pub fn primary(&self) -> &PrimaryLanguage {
		unsafe {
			PrimaryLanguage::new_unchecked(&self.as_bytes()[..self.primary_len()])
		}
	}

	/// Return the extended language subtags.
	/// 
	/// Extended language subtags are used to identify certain specially
	/// selected languages that, for various historical and compatibility
	/// reasons, are closely identified with or tagged using an existing
	/// primary language subtag.
	pub fn extension(&self) -> Option<&LanguageExtension> {
		let bytes = self.as_bytes();
		let i = self.primary_len()+1;
		if i < bytes.len() {
			unsafe {
				Some(LanguageExtension::new_unchecked(&self.as_bytes()[i..]))
			}
		} else {
			None
		}
	}

	/// Return an iterator to the extended language subtags.
	pub fn extension_subtags(&self) -> LanguageExtensionIter {
		LanguageExtensionIter {
			bytes: &self.as_bytes()[(self.primary_len()+1)..],
			i: 0
		}
	}
}

macro_rules! iterator {
	($collection:ident, $id:ident, $item:ident, $offset:literal) => {
		pub struct $id<'a> {
			bytes: &'a [u8],
			i: usize
		}
		
		impl<'a> Iterator for $id<'a> {
			type Item = &'a $item;
		
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
						Some($item::new_unchecked(&self.bytes[offset..self.i]))
					}
				} else {
					None
				}
			}
		}
		
		impl $collection {
			pub fn iter(&self) -> $id {
				$id {
					bytes: self.as_bytes(),
					i: $offset
				}
			}
		}
		
		impl<'a> IntoIterator for &'a $collection {
			type Item = &'a $item;
			type IntoIter = $id<'a>;
		
			fn into_iter(self) -> $id<'a> {
				$id {
					bytes: self.as_bytes(),
					i: $offset
				}
			}
		}
	};
}

iterator!(LanguageExtension, LanguageExtensionIter, ExtendedLangTag, 0);
iterator!(Variants, VariantsIter, Variant, 0);
iterator!(Extension, ExtensionIter, ExtensionSubtag, 2);
iterator!(PrivateUseSubtags, PrivateUseSubtagsIter, PrivateUseSubtag, 2);

pub struct ExtensionsIter<'a> {
	bytes: &'a [u8],
	i: usize,
	current_id: u8
}

impl<'a> Iterator for ExtensionsIter<'a> {
	type Item = (u8, &'a ExtensionSubtag);

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
						return Some((self.current_id, ExtensionSubtag::new_unchecked(&self.bytes[offset..self.i])))
					}
				} else {
					self.current_id = self.bytes[offset];
				}
			}
		} else {
			None
		}
	}
}