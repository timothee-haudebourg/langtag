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
	ops::{
		Range,
		Deref
	}
};

mod error;
mod parse;
mod langtag;
mod privateuse;
mod grandfathered;

pub use error::*;
pub use self::langtag::*;
pub use privateuse::*;
pub use grandfathered::*;

pub enum LanguageTag<'a, T: ?Sized = [u8]> {
	Normal(LangTag<&'a T>),
	PrivateUse(&'a PrivateUseTag<T>),
	Grandfathered(GrandfatheredTag)
}

pub enum LanguageTagBuf<T = Vec<u8>> {
	Normal(LangTag<T>),
	PrivateUse(PrivateUseTag<T>),
	Grandfathered(GrandfatheredTag)
}

impl<T: AsRef<[u8]>> LanguageTagBuf<T> {
	#[inline]
	pub fn new(t: T) -> Result<LanguageTagBuf<T>, Error> {
		match GrandfatheredTag::new(t) {
			Ok(tag) => Ok(LanguageTagBuf::Grandfathered(tag)),
			Err(t) => match PrivateUseTag::new(t) {
				Ok(tag) => Ok(LanguageTagBuf::PrivateUse(tag)),
				Err(t) => {
					Ok(LanguageTagBuf::Normal(LangTag::new(t)?))
				}
			}
		}
	}
}

impl LanguageTagBuf {
	#[inline]
	pub fn parse_copy<T: AsRef<[u8]> + ?Sized>(t: &T) -> Result<LanguageTagBuf, Error> {
		let mut buffer = Vec::new();
		buffer.copy_from_slice(t.as_ref());
		Self::new(buffer)
	}
}

macro_rules! language_tag_impl {
	() => {
		#[inline]
		pub fn as_bytes(&self) -> &[u8] {
			match self {
				Self::Normal(tag) => tag.as_bytes(),
				Self::PrivateUse(tag) => tag.as_bytes(),
				Self::Grandfathered(tag) => tag.as_bytes()
			}
		}
	
		#[inline]
		pub fn as_str(&self) -> &str {
			match self {
				Self::Normal(tag) => tag.as_str(),
				Self::PrivateUse(tag) => tag.as_str(),
				Self::Grandfathered(tag) => tag.as_str()
			}
		}
	
		#[inline]
		pub fn is_normal(&self) -> bool {
			match self {
				Self::Normal(_) => true,
				_ => false
			}
		}
	
		#[inline]
		pub fn is_private_use(&self) -> bool {
			match self {
				Self::PrivateUse(_) => true,
				_ => false
			}
		}
	
		#[inline]
		pub fn is_grandfathered(&self) -> bool {
			match self {
				Self::Grandfathered(_) => true,
				_ => false
			}
		}
	
		#[inline]
		pub fn language(&self) -> Option<&Language> {
			match self {
				Self::Normal(tag) => Some(tag.language()),
				Self::PrivateUse(_) => None,
				Self::Grandfathered(tag) => tag.language()
			}
		}
	
		#[inline]
		pub fn script(&self) -> Option<&Script> {
			match self {
				Self::Normal(tag) => tag.script(),
				Self::PrivateUse(_) => None,
				Self::Grandfathered(_) => None
			}
		}
	
		#[inline]
		pub fn region(&self) -> Option<&Region> {
			match self {
				Self::Normal(tag) => tag.region(),
				Self::PrivateUse(_) => None,
				Self::Grandfathered(_) => None
			}
		}
	
		#[inline]
		pub fn variants(&self) -> &Variants {
			match self {
				Self::Normal(tag) => tag.variants(),
				Self::PrivateUse(_) => Variants::empty(),
				Self::Grandfathered(_) => Variants::empty()
			}
		}
	
		#[inline]
		pub fn extensions(&self) -> &Extensions {
			match self {
				Self::Normal(tag) => tag.extensions(),
				Self::PrivateUse(_) => Extensions::empty(),
				Self::Grandfathered(_) => Extensions::empty()
			}
		}
	
		#[inline]
		pub fn private_use_subtags(&self) -> &PrivateUseSubtags {
			match self {
				Self::Normal(tag) => tag.private_use_subtags(),
				Self::PrivateUse(tag) => tag.subtags(),
				Self::Grandfathered(_) => PrivateUseSubtags::empty()
			}
		}
	}
}

impl <'a, T: AsRef<[u8]> + ?Sized> LanguageTag<'a, T> {
	language_tag_impl!();
}

impl <T: AsRef<[u8]>> LanguageTagBuf<T> {
	language_tag_impl!();
}

impl<'a> LanguageTag<'a> {
	#[inline]
	pub fn parse<T: AsRef<[u8]> + ?Sized>(t: &'a T) -> Result<LanguageTag<'a>, Error> {
		match GrandfatheredTag::new(t) {
			Ok(tag) => Ok(LanguageTag::Grandfathered(tag)),
			Err(_) => match PrivateUseTag::parse(t) {
				Ok(tag) => Ok(LanguageTag::PrivateUse(tag)),
				Err(_) => {
					Ok(LanguageTag::Normal(LangTag::parse(t)?))
				}
			}
		}
	}
}

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

impl<'a, T: AsRef<[u8]> + ?Sized> Eq for LanguageTag<'a, T> { }

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

pub type LangTagBuf = LangTag<Vec<u8>>;
pub type PrivateUseTagBuf = PrivateUseTag<Vec<u8>>;

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
	($(#[ doc = $doc:tt ])* $parser:ident, $multi:expr, $id:ident, $err:ident) => {
		$(#[doc=$doc])*
		pub struct $id<T: ?Sized = [u8]> {
			data: T
		}

		impl<T: AsRef<[u8]>> $id<T> {
			#[inline]
			pub fn new(t: T) -> Result<$id<T>, Error> {
				let bytes = t.as_ref();
				
				if ($multi || bytes.len() > 0) && crate::parse::$parser(bytes, 0) == bytes.len() {
					Ok($id {
						data: t
					})
				} else {
					Err(Error::$err)
				}
			}

			#[inline]
			pub unsafe fn new_unchecked(t: T) -> $id<T> {
				$id {
					data: t
				}
			}
		}

		impl $id<[u8]> {
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

			pub unsafe fn parse_unchecked<'a, T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> &'a $id<[u8]> {
				&*(bytes.as_ref() as *const [u8] as *const $id<[u8]>)
			}
		}

		impl $id<Vec<u8>> {
			pub fn parse_copy<'a, T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> Result<$id<Vec<u8>>, Error> {
				let mut buffer = Vec::new();
				buffer.copy_from_slice(bytes.as_ref());
				$id::new(buffer)
			}

			pub unsafe fn parse_copy_unchecked<'a, T: AsRef<[u8]> + ?Sized>(bytes: &'a T) -> $id<Vec<u8>> {
				let mut buffer = Vec::new();
				buffer.copy_from_slice(bytes.as_ref());
				$id::new_unchecked(buffer)
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> $id<T> {
			#[inline]
			pub fn len(&self) -> usize {
				self.as_bytes().len()
			}

			#[inline]
			pub fn as_bytes(&self) -> &[u8] {
				self.data.as_ref()
			}

			#[inline]
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
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				fmt::Display::fmt(self.as_str(), f)
			}
		}

		impl<T: AsRef<[u8]> + ?Sized> fmt::Debug for $id<T> {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				fmt::Debug::fmt(self.as_str(), f)
			}
		}
	};
}

component! {
	/// Primary and extended language subtags.
	/// 
	/// This type represents the primary language subtag (first subtag in a
	/// language tag) and the extended language subtags associated with it.
	language, false, Language, InvalidLangage
}

component! {
	/// Primary language subtag.
	/// 
	/// The primary language subtag is the first subtag in a language tag.
	primary_language, false, PrimaryLanguage, InvalidPrimaryLangage
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
	extlang, false, LanguageExtension, InvalidLangageExtension
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

component! {
	/// Single variant subtag.
	/// 
	/// Variant subtags are used to indicate additional, well-recognized
	/// variations that define a language or its dialects that are not
	/// covered by other available subtags.
	variant, false, Variant, InvalidVariant
}

component! {
	/// List of variant subtags.
	/// 
	/// Represents a list of variant subtags separated by a `-` character
	/// as found in a language tag.
	variants, true, Variants, InvalidVariants
}

component! {
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
	extension, false, Extension, InvalidExtension
}

component! {
	/// Single extension subtag.
	/// 
	/// Extension subtag found in a language tag extension.
	extension_subtag, false, ExtensionSubtag, InvalidExtensionSubtag
}

component! {
	/// List of extension subtags.
	/// 
	/// A list of language tag extension, separated by a `-` character.
	/// Individual extensions are represented by the [`Extension`] type,
	/// while extension subtags are represented by the [`ExtensionSubtag`]
	/// type.
	extensions, true, Extensions, InvalidExtensions
}

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

impl Language {
	/// Return the length (in bytes) of the primary subtag.
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
			PrimaryLanguage::parse_unchecked(&self.as_bytes()[..self.primary_len()])
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
				Some(LanguageExtension::parse_unchecked(&self.as_bytes()[i..]))
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

impl Extensions {
	/// The empty list of extension (an empty string).
	pub fn empty() -> &'static Extensions {
		unsafe {
			Extensions::parse_unchecked(b"")
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
						Some($item::parse_unchecked(&self.bytes[offset..self.i]))
					}
				} else {
					None
				}
			}
		}
		
		impl $collection {
			/// Empty list (the empty string).
			pub fn empty() -> &'static $collection {
				unsafe {
					$collection::parse_unchecked(b"")
				}
			}

			/// Checks if the list is empty.
			pub fn is_empty(&self) -> bool {
				self.as_bytes().is_empty()
			}

			/// Returns the first subtag of the list (if any).
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

/// Mutable reference to the variants of a language tag.
pub struct VariantsMut<'a> {
	/// Language tag buffer.
	buffer: &'a mut Vec<u8>,

	/// Language tag parsing data.
	p: &'a mut parse::ParsedLangTag
}

impl<'a> VariantsMut<'a> {
	/// Checks if the list of variants is empty.
	pub fn is_empty(&self) -> bool {
		self.p.variant_end > self.p.region_end
	}

	/// Returns the first variant subtag of the list (if any).
	pub fn first(&self) -> Option<&Variant> {
		if self.is_empty() {
			None
		} else {
			let mut i = self.p.region_end;

			while i < self.buffer.len() && self.buffer[i] != b'-' {
				i += 1
			}

			unsafe {
				Some(Variant::parse_unchecked(&self.buffer[i..self.p.variant_end]))
			}
		}
	}

	/// Returns the last variant subtag of the list (if any).
	pub fn last(&self) -> Option<&Variant> {
		if self.is_empty() {
			None
		} else {
			let mut i = self.p.variant_end-1;

			while i > 1 && self.buffer[i-1] != b'-' {
				i -= 1
			}

			unsafe {
				Some(Variant::parse_unchecked(&self.buffer[i..self.p.variant_end]))
			}
		}
	}

	/// Add a new variant subtag at the end.
	pub fn push(&mut self, variant: &Variant) {
		let bytes = variant.as_bytes();

		let mut i = self.p.variant_end;
		replace(self.buffer, i..i, b"-");
		i += 1;
		replace(self.buffer, i..i, bytes);

		let len = bytes.len() + 1;
		self.p.variant_end += len;
		self.p.extension_end += len;
		self.p.privateuse_end += len;
	}

	/// Removes and return the last variant subtag of the list (if any).
	pub fn pop(&mut self) -> Option<Variant<Vec<u8>>> {
		match self.last() {
			Some(last) => {
				let mut new_end = self.p.variant_end - last.len();

				let copy = unsafe {
					Variant::parse_copy_unchecked(&self.buffer[new_end..self.p.variant_end])
				};

				if new_end > self.p.region_end {
					new_end -= 1
				}

				replace(self.buffer, new_end..self.p.variant_end, b"");

				let len = self.p.variant_end - new_end;
				self.p.variant_end -= len;
				self.p.extension_end -= len;
				self.p.privateuse_end -= len;

				Some(copy)
			},
			None => None
		}
	}
}

iterator!(Extension, ExtensionIter, ExtensionSubtag, 2);

iterator!(PrivateUseSubtags, PrivateUseSubtagsIter, PrivateUseSubtag, 2);

/// Extensions iterator.
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
						return Some((self.current_id, ExtensionSubtag::parse_unchecked(&self.bytes[offset..self.i])))
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

		if range_len > content.len() { // shrink
			for i in 0..tail_len {
				buffer[new_end + i] = buffer[range.end + i];
			}

			buffer.resize(new_end + tail_len, 0);
		} else { // grow
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