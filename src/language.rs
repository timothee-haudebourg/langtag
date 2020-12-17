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
	component,
	iterator,
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
	#[inline]
	pub fn extension(&self) -> Option<&LanguageExtension> {
		let bytes = self.as_bytes();
		let i = self.primary_len()+1;
		if i < bytes.len() {
			unsafe {
				Some(LanguageExtension::parse_unchecked(&self.as_bytes()[i..]))
			}
		} else {
			None
		}
	}

	/// Return an iterator to the extended language subtags.
	#[inline]
	pub fn extension_subtags(&self) -> LanguageExtensionIter {
		LanguageExtensionIter {
			bytes: &self.as_bytes()[(self.primary_len()+1)..],
			i: 0
		}
	}
}

pub struct LanguageMut<'a> {
	/// Language tag buffer.
	pub(crate) buffer: &'a mut Vec<u8>,

	/// Language tag parsing data.
	pub(crate) p: &'a mut parse::ParsedLangTag
}

impl<'a> LanguageMut<'a> {
	#[inline]
	pub fn set_primary(&mut self, primary: &PrimaryLanguage) {
		unimplemented!() // TODO
	}

	#[inline]
	pub fn set_extension(&mut self, ext: &LanguageExtension) {
		unimplemented!() // TODO
	}

	#[inline]
	pub fn extension_mut(&mut self) -> LanguageExtensionMut {
		LanguageExtensionMut {
			buffer: self.buffer,
			p: self.p
		}
	}
}

pub struct LanguageExtensionMut<'a> {
	/// Language tag buffer.
	buffer: &'a mut Vec<u8>,

	/// Language tag parsing data.
	p: &'a mut parse::ParsedLangTag
}

impl<'a> LanguageExtensionMut<'a> {
	#[inline]
	pub fn insert(&mut self, tag: &ExtendedLangTag) {
		unimplemented!() // TODO
	}

	#[inline]
	pub fn remove<T: AsRef<[u8]>>(&mut self, tag: &T) {
		unimplemented!() // TODO
	}
}