use crate::{
	parse, Error, Extensions, ExtensionsMut, Language, LanguageMut, PrivateUseSubtags,
	PrivateUseSubtagsMut, Region, Script, Variants, VariantsMut,
};
use std::{
	fmt,
	hash::{Hash, Hasher},
};

/// Normal language subtag.
///
/// The language subtag can be modified when the internal buffer type (`T`) is `Vec<u8>`.
#[derive(Clone, Copy)]
pub struct LangTag<T> {
	p: parse::ParsedLangTag,
	data: T,
}

impl<'a> LangTag<&'a [u8]> {
	/// Parse a normal language tag.
	#[inline]
	pub fn parse<T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> Result<LangTag<&'a [u8]>, Error> {
		let bytes = bytes.as_ref();
		let p = parse::langtag(bytes, 0)?;

		if p.len() > 0 && p.len() == bytes.len() {
			Ok(LangTag { p, data: bytes })
		} else {
			Err(Error::InvalidLangTag)
		}
	}
}

impl LangTag<Vec<u8>> {
	/// Parse and copy a normal language tag.
	///
	/// The returned normal language tag owns its buffer.
	#[inline]
	pub fn parse_copy<T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<LangTag<Vec<u8>>, Error> {
		let bytes = bytes.as_ref();
		let mut buffer = Vec::new();
		buffer.resize(bytes.len(), 0);
		buffer.copy_from_slice(bytes);
		Self::new(buffer).map_err(|(e, _)| e)
	}
}

impl<T: AsRef<[u8]>> LangTag<T> {
	/// Create a new normal language tag by parsing and using the given buffer.
	#[inline]
	pub fn new(data: T) -> Result<LangTag<T>, (Error, T)> {
		let bytes = data.as_ref();
		match parse::langtag(bytes, 0) {
			Ok(p) => {
				if p.len() > 0 && p.len() == bytes.len() {
					Ok(LangTag { p, data })
				} else {
					Err((Error::InvalidLangTag, data))
				}
			}
			Err(e) => Err((e, data)),
		}
	}

	/// Consume the tag and returns its internal buffer along with the parsing metadata.
	#[inline]
	pub fn into_raw_parts(self) -> (T, parse::ParsedLangTag) {
		(self.data, self.p)
	}

	/// Create a new normal language tag using `p` as parsing metadata.
	///
	/// ## Safety
	/// The input data is not checked for well-formedness, which must be ensred by the caller.
	#[inline]
	pub unsafe fn from_raw_parts(data: T, p: parse::ParsedLangTag) -> LangTag<T> {
		LangTag { p, data }
	}

	/// Returns a reference to the tag's buffer.
	#[inline]
	pub fn inner(&self) -> &T {
		&self.data
	}

	/// Returns a copy of the parsing metadata.
	#[inline]
	pub fn parsing_data(&self) -> parse::ParsedLangTag {
		self.p
	}

	/// Returns the bytes representation of the tag.
	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.data.as_ref()
	}

	/// Returns the string representation of the tag.
	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}

	/// Get the language subtags.
	#[inline]
	pub fn language(&self) -> &Language {
		unsafe { Language::parse_unchecked(&self.data.as_ref()[0..self.p.language_end]) }
	}

	/// Get the script subtag, if any.
	#[inline]
	pub fn script(&self) -> Option<&Script> {
		if self.p.language_end < self.p.script_end {
			unsafe {
				Some(Script::parse_unchecked(
					&self.data.as_ref()[(self.p.language_end + 1)..self.p.script_end],
				))
			}
		} else {
			None
		}
	}

	/// Get the region subtag, if any.
	#[inline]
	pub fn region(&self) -> Option<&Region> {
		if self.p.script_end < self.p.region_end {
			unsafe {
				Some(Region::parse_unchecked(
					&self.data.as_ref()[(self.p.script_end + 1)..self.p.region_end],
				))
			}
		} else {
			None
		}
	}

	/// Get the variant subtags.
	#[inline]
	pub fn variants(&self) -> &Variants {
		unsafe {
			Variants::parse_unchecked(
				&self.data.as_ref()[(self.p.region_end + 1)..self.p.variant_end],
			)
		}
	}

	/// Get the extension subtags.
	#[inline]
	pub fn extensions(&self) -> &Extensions {
		unsafe {
			Extensions::parse_unchecked(
				&self.data.as_ref()[(self.p.variant_end + 1)..self.p.extension_end],
			)
		}
	}

	/// Get the private use subtags.
	#[inline]
	pub fn private_use_subtags(&self) -> &PrivateUseSubtags {
		unsafe {
			PrivateUseSubtags::parse_unchecked(
				&self.data.as_ref()[(self.p.extension_end + 1)..self.p.privateuse_end],
			)
		}
	}
}

impl<T: AsMut<Vec<u8>>> LangTag<T> {
	/// Get and modify the language subtags.
	#[inline]
	pub fn language_mut(&mut self) -> LanguageMut {
		LanguageMut {
			buffer: self.data.as_mut(),
			p: &mut self.p,
		}
	}

	/// Set the language subtags.
	#[inline]
	pub fn set_language(&mut self, lang: &Language) {
		let lang = lang.as_bytes();
		crate::replace(self.data.as_mut(), 0..self.p.language_end, lang);
		let new_end = lang.len();
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

	/// Set the script subtag.
	#[inline]
	pub fn set_script(&mut self, script: Option<&Script>) {
		let new_end = match script {
			Some(script) => {
				let script = script.as_bytes();
				crate::replace(
					self.data.as_mut(),
					self.p.language_end..self.p.script_end,
					script,
				);
				crate::replace(
					self.data.as_mut(),
					self.p.language_end..self.p.language_end,
					&[b'-'],
				);
				self.p.language_end + 1 + script.len()
			}
			None => {
				crate::replace(
					self.data.as_mut(),
					self.p.language_end..self.p.script_end,
					&[],
				);
				self.p.language_end
			}
		};

		if self.p.script_end < new_end {
			let diff = new_end - self.p.script_end;
			self.p.region_end += diff;
			self.p.variant_end += diff;
			self.p.extension_end += diff;
			self.p.privateuse_end += diff;
		} else {
			let diff = self.p.script_end - new_end;
			self.p.region_end -= diff;
			self.p.variant_end -= diff;
			self.p.extension_end -= diff;
			self.p.privateuse_end -= diff;
		}
		self.p.script_end = new_end;
	}

	/// Set the region subtag.
	#[inline]
	pub fn set_region(&mut self, region: Option<&Region>) {
		let new_end = match region {
			Some(region) => {
				let region = region.as_bytes();
				crate::replace(
					self.data.as_mut(),
					self.p.script_end..self.p.region_end,
					region,
				);
				crate::replace(
					self.data.as_mut(),
					self.p.script_end..self.p.script_end,
					&[b'-'],
				);
				self.p.script_end + 1 + region.len()
			}
			None => {
				crate::replace(
					self.data.as_mut(),
					self.p.script_end..self.p.region_end,
					&[],
				);
				self.p.script_end
			}
		};

		if self.p.region_end < new_end {
			let diff = new_end - self.p.region_end;
			self.p.variant_end += diff;
			self.p.extension_end += diff;
			self.p.privateuse_end += diff;
		} else {
			let diff = self.p.region_end - new_end;
			self.p.variant_end -= diff;
			self.p.extension_end -= diff;
			self.p.privateuse_end -= diff;
		}
		self.p.region_end = new_end;
	}

	/// Get and modify the variant subtags.
	#[inline]
	pub fn variants_mut(&mut self) -> VariantsMut {
		VariantsMut {
			buffer: self.data.as_mut(),
			p: &mut self.p,
		}
	}

	/// Get and modify the extension subtags.
	#[inline]
	pub fn extensions_mut(&mut self) -> ExtensionsMut {
		ExtensionsMut {
			buffer: self.data.as_mut(),
			p: &mut self.p,
		}
	}

	/// Get and modify the private use subtags.
	#[inline]
	pub fn private_use_subtags_mut(&mut self) -> PrivateUseSubtagsMut {
		PrivateUseSubtagsMut {
			buffer: self.data.as_mut(),
			offset: self.p.extension_end,
		}
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]>> PartialEq<LangTag<U>> for LangTag<T> {
	#[inline]
	fn eq(&self, other: &LangTag<U>) -> bool {
		crate::case_insensitive_eq(self.data.as_ref(), other.data.as_ref())
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]> + ?Sized> PartialEq<U> for LangTag<T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		crate::case_insensitive_eq(self.as_bytes(), other.as_ref())
	}
}

impl<T: AsRef<[u8]>> Eq for LangTag<T> {}

impl<T: AsRef<[u8]>> Hash for LangTag<T> {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		crate::case_insensitive_hash(self.data.as_ref(), h)
	}
}

impl<T: AsRef<[u8]>> fmt::Display for LangTag<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]>> fmt::Debug for LangTag<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}
