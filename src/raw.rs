use std::{
	fmt,
	hash::{
		Hash,
		Hasher
	}
};
use crate::{
	parse,
	GrandfatheredTag,
	Language,
	Script,
	Region,
	Variants,
	Extensions,
	PrivateUseSubtags,
	case_insensitive_eq
};

pub enum LanguageTag<T> {
	Normal(LangTag<T>),
	PrivateUse(PrivateUseTag<T>),
	Grandfathered(GrandfatheredTag)
}

impl<T: AsRef<[u8]>> LanguageTag<T> {
	#[inline]
	pub fn new(t: T) -> Result<LanguageTag<T>, parse::Error> {
		match GrandfatheredTag::new(t) {
			Ok(tag) => Ok(LanguageTag::Grandfathered(tag)),
			Err(t) => match PrivateUseTag::new(t) {
				Ok(tag) => Ok(LanguageTag::PrivateUse(tag)),
				Err(t) => {
					Ok(LanguageTag::Normal(LangTag::new(t)?))
				}
			}
		}
	}

	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		match self {
			LanguageTag::Normal(tag) => tag.as_bytes(),
			LanguageTag::PrivateUse(tag) => tag.as_bytes(),
			LanguageTag::Grandfathered(tag) => tag.as_bytes()
		}
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		match self {
			LanguageTag::Normal(tag) => tag.as_str(),
			LanguageTag::PrivateUse(tag) => tag.as_str(),
			LanguageTag::Grandfathered(tag) => tag.as_str()
		}
	}

	#[inline]
	pub fn language(&self) -> Option<&Language> {
		match self {
			LanguageTag::Normal(tag) => Some(tag.language()),
			LanguageTag::PrivateUse(_) => None,
			LanguageTag::Grandfathered(tag) => tag.language()
		}
	}
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for LanguageTag<T> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<T: AsRef<[u8]>> AsRef<str> for LanguageTag<T> {
	#[inline]
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]>> PartialEq<U> for LanguageTag<T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		case_insensitive_eq(self.as_bytes(), other.as_ref())
	}
}

impl<T: AsRef<[u8]>> Eq for LanguageTag<T> { }

impl<T: AsRef<[u8]>> fmt::Display for LanguageTag<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]>> fmt::Debug for LanguageTag<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}

pub struct LangTag<T> {
	p: parse::ParsedLangTag,
	data: T
}

impl<T: AsRef<[u8]>> LangTag<T> {
	#[inline]
	pub fn new(data: T) -> Result<LangTag<T>, parse::Error> {
		let bytes = data.as_ref();
		let p = parse::langtag(bytes, 0)?;

		if p.len() > 0 && p.len() == bytes.len() {
			Ok(LangTag {
				p,
				data
			})
		} else {
			Err(parse::Error::MalformedLangTag)
		}
	}

	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.data.as_ref()
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}

	#[inline]
	pub fn language(&self) -> &Language {
		unsafe {
			Language::new_unchecked(&self.data.as_ref()[0..self.p.language_end])
		}
	}

	#[inline]
	pub fn script(&self) -> Option<&Script> {
		if self.p.language_end < self.p.script_end {
			unsafe {
				Some(Script::new_unchecked(&self.data.as_ref()[self.p.language_end..self.p.script_end]))
			}
		} else {
			None
		}
	}

	#[inline]
	pub fn region(&self) -> Option<&Region> {
		if self.p.script_end < self.p.region_end {
			unsafe {
				Some(Region::new_unchecked(&self.data.as_ref()[self.p.script_end..self.p.region_end]))
			}
		} else {
			None
		}
	}

	#[inline]
	pub fn variants(&self) -> &Variants {
		unsafe {
			Variants::new_unchecked(&self.data.as_ref()[self.p.region_end..self.p.variant_end])
		}
	}

	#[inline]
	pub fn extensions(&self) -> &Extensions {
		unsafe {
			Extensions::new_unchecked(&self.data.as_ref()[self.p.variant_end..self.p.extension_end])
		}
	}

	#[inline]
	pub fn private_use_subtags(&self) -> &PrivateUseSubtags {
		unsafe {
			PrivateUseSubtags::new_unchecked(&self.data.as_ref()[self.p.extension_end..self.p.privateuse_end])
		}
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]>> PartialEq<LangTag<U>> for LangTag<T> {
	fn eq(&self, other: &LangTag<U>) -> bool {
		crate::case_insensitive_eq(self.data.as_ref(), other.data.as_ref())
	}
}

impl<T: AsRef<[u8]>> Eq for LangTag<T> {}

impl<T: AsRef<[u8]>> Hash for LangTag<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		crate::case_insensitive_hash(self.data.as_ref(), h)
	}
}

pub struct PrivateUseTag<T> {
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
	pub fn as_ref(&self) -> &PrivateUseSubtags {
		unsafe {
			PrivateUseSubtags::new_unchecked(self.as_bytes())
		}
	}
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for PrivateUseTag<T> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]>> PartialEq<U> for PrivateUseTag<T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		crate::case_insensitive_eq(self.data.as_ref(), other.as_ref())
	}
}

impl<T: AsRef<[u8]>> Eq for PrivateUseTag<T> {}

impl<T: AsRef<[u8]>> Hash for PrivateUseTag<T> {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		crate::case_insensitive_hash(self.data.as_ref(), h)
	}
}