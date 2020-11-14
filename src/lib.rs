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

mod parse;
pub mod privateusetag;
pub mod langtag;
mod grandfathered;

pub use grandfathered::*;

pub enum RawLanguageTag<T> {
	Normal(langtag::LangTag<T>),
	PrivateUse(privateusetag::PrivateUseTag<T>),
	Grandfathered(GrandfatheredTag)
}

impl<'a> RawLanguageTag<&'a [u8]> {
	#[inline]
	pub fn parse<T: AsRef<[u8]> + ?Sized>(t: &'a T) -> Result<RawLanguageTag<&'a [u8]>, parse::Error> {
		Self::new(t.as_ref())
	}
}

impl<T: AsRef<[u8]>> RawLanguageTag<T> {
	#[inline]
	pub fn new(t: T) -> Result<RawLanguageTag<T>, parse::Error> {
		match GrandfatheredTag::new(t) {
			Ok(tag) => Ok(RawLanguageTag::Grandfathered(tag)),
			Err(t) => match privateusetag::PrivateUseTag::new(t) {
				Ok(tag) => Ok(RawLanguageTag::PrivateUse(tag)),
				Err(t) => {
					Ok(RawLanguageTag::Normal(langtag::LangTag::new(t)?))
				}
			}
		}
	}

	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		match self {
			RawLanguageTag::Normal(tag) => tag.as_bytes(),
			RawLanguageTag::PrivateUse(tag) => tag.as_bytes(),
			RawLanguageTag::Grandfathered(tag) => tag.as_bytes()
		}
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		match self {
			RawLanguageTag::Normal(tag) => tag.as_str(),
			RawLanguageTag::PrivateUse(tag) => tag.as_str(),
			RawLanguageTag::Grandfathered(tag) => tag.as_str()
		}
	}

	#[inline]
	pub fn language(&self) -> Option<&Language> {
		match self {
			RawLanguageTag::Normal(tag) => Some(tag.language()),
			RawLanguageTag::PrivateUse(_) => None,
			RawLanguageTag::Grandfathered(tag) => tag.language()
		}
	}
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for RawLanguageTag<T> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<T: AsRef<[u8]>> AsRef<str> for RawLanguageTag<T> {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]>> PartialEq<U> for RawLanguageTag<T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		case_insensitive_eq(self.as_bytes(), other.as_ref())
	}
}

impl<T: AsRef<[u8]>> Eq for RawLanguageTag<T> { }

impl<T: AsRef<[u8]>> fmt::Display for RawLanguageTag<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]>> fmt::Debug for RawLanguageTag<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}

pub type LangTag<'a> = langtag::LangTag<&'a [u8]>;
pub type LangTagBuf = langtag::LangTag<Vec<u8>>;

pub type PrivateUseTag<'a> = privateusetag::PrivateUseTag<&'a [u8]>;
pub type PrivateUseTagBuf = privateusetag::PrivateUseTag<Vec<u8>>;

pub type LanguageTag<'a> = RawLanguageTag<&'a [u8]>;
pub type LanguageTagBuf = RawLanguageTag<Vec<u8>>;

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
	($parser:ident, $id:ident, $err:ident) => {
		pub struct $err;

		pub struct $id {
			data: [u8]
		}

		impl $id {
			#[inline]
			pub fn new<B: AsRef<[u8]> + ?Sized>(bytes: &B) -> Result<&$id, $err> {
				let bytes = bytes.as_ref();
				
				if bytes.len() > 0 && crate::parse::$parser(bytes.as_ref(), 0) == bytes.len() {
					Ok(unsafe { Self::new_unchecked(bytes) })
				} else {
					Err($err)
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
			type Error = $err;

			#[inline]
			fn try_from(b: &'a str) -> Result<&'a $id, $err> {
				$id::new(b)
			}
		}

		impl<'a> TryFrom<&'a [u8]> for &'a $id {
			type Error = $err;

			#[inline]
			fn try_from(b: &'a [u8]) -> Result<&'a $id, $err> {
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

component!(language, Language, InvalidLangage);
component!(primary_language, PrimaryLanguage, InvalidPrimaryLangage);
component!(script, Script, InvalidScript);
component!(region, Region, InvalidRegion);
component!(variant, Variant, InvalidVariant);
component!(extension, Extension, InvalidExtension);
component!(privateuse, PrivateUse, InvalidPrivateUse);

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

	pub fn primary(&self) -> &PrimaryLanguage {
		unsafe {
			PrimaryLanguage::new_unchecked(&self.as_bytes()[0..self.primary_len()])
		}
	}
}