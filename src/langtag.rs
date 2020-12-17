use std::{
	fmt,
	hash::{
		Hash,
		Hasher
	}
};
use crate::{
	Error,
	Language,
	Script,
	Region,
	Variants,
	VariantsMut,
	Extensions,
	PrivateUseSubtags
};

pub struct LangTag<T> {
	p: crate::parse::ParsedLangTag,
	data: T
}

impl<'a> LangTag<&'a [u8]> {
	pub fn parse<T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> Result<LangTag<&'a [u8]>, Error> {
		let bytes = bytes.as_ref();
		let p = crate::parse::langtag(bytes, 0)?;

		if p.len() > 0 && p.len() == bytes.len() {
			Ok(LangTag {
				p,
				data: bytes
			})
		} else {
			Err(Error::InvalidLangTag)
		}
	}
}

impl<T: AsRef<[u8]>> LangTag<T> {
	#[inline]
	pub fn new(data: T) -> Result<LangTag<T>, Error> {
		let bytes = data.as_ref();
		let p = crate::parse::langtag(bytes, 0)?;

		if p.len() > 0 && p.len() == bytes.len() {
			Ok(LangTag {
				p,
				data
			})
		} else {
			Err(Error::InvalidLangTag)
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
			Language::parse_unchecked(&self.data.as_ref()[0..self.p.language_end])
		}
	}

	#[inline]
	pub fn script(&self) -> Option<&Script> {
		if self.p.language_end < self.p.script_end {
			unsafe {
				Some(Script::parse_unchecked(&self.data.as_ref()[(self.p.language_end+1)..self.p.script_end]))
			}
		} else {
			None
		}
	}

	#[inline]
	pub fn region(&self) -> Option<&Region> {
		if self.p.script_end < self.p.region_end {
			unsafe {
				Some(Region::parse_unchecked(&self.data.as_ref()[(self.p.script_end+1)..self.p.region_end]))
			}
		} else {
			None
		}
	}

	#[inline]
	pub fn variants(&self) -> &Variants {
		unsafe {
			Variants::parse_unchecked(&self.data.as_ref()[(self.p.region_end+1)..self.p.variant_end])
		}
	}

	#[inline]
	pub fn extensions(&self) -> &Extensions {
		unsafe {
			Extensions::parse_unchecked(&self.data.as_ref()[(self.p.variant_end+1)..self.p.extension_end])
		}
	}

	#[inline]
	pub fn private_use_subtags(&self) -> &PrivateUseSubtags {
		unsafe {
			PrivateUseSubtags::parse_unchecked(&self.data.as_ref()[(self.p.extension_end+1)..self.p.privateuse_end])
		}
	}
}

impl<T: AsMut<Vec<u8>>> LangTag<T> {
	#[inline]
	pub fn variants_mut(&mut self) -> VariantsMut {
		VariantsMut {
			buffer: self.data.as_mut(),
			p: &mut self.p
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

impl<T: AsRef<[u8]>> fmt::Display for LangTag<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]>> fmt::Debug for LangTag<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}