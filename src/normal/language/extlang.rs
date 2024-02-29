use std::hash::Hash;

use static_regular_grammar::RegularGrammar;

use crate::utils::{self, str_eq};

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
#[derive(RegularGrammar)]
#[grammar(
	file = "src/grammar.abnf",
	entry_point = "extlang",
	cache = "automata/extlang.aut.cbor"
)]
#[grammar(sized(
	LanguageExtensionBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
#[cfg_attr(feature = "serde", grammar(serde))]
pub struct LanguageExtension(str);

impl LanguageExtension {
	pub fn iter(&self) -> LanguageExtensionIter {
		LanguageExtensionIter::new(&self.0)
	}
}

impl PartialEq for LanguageExtension {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for LanguageExtension {}

str_eq!(LanguageExtension);
str_eq!(LanguageExtensionBuf);

impl PartialOrd for LanguageExtension {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for LanguageExtension {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for LanguageExtension {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}

#[derive(Default)]
pub struct LanguageExtensionIter<'a> {
	data: &'a str,
	offset: usize,
}

impl<'a> LanguageExtensionIter<'a> {
	fn new(data: &'a str) -> Self {
		Self { data, offset: 0 }
	}

	pub fn empty() -> Self {
		Self::default()
	}
}

impl<'a> Iterator for LanguageExtensionIter<'a> {
	type Item = &'a ExtendedLangTag;

	fn next(&mut self) -> Option<Self::Item> {
		if self.offset < self.data.len() {
			let end = super::super::find_segment_end(self.data, self.offset);
			let item = unsafe { ExtendedLangTag::new_unchecked(&self.data[self.offset..end]) };
			self.offset = end + 1;
			Some(item)
		} else {
			None
		}
	}
}

/// Single extended language subtag.
///
/// Extended language subtags are used to identify certain specially
/// selected languages that, for various historical and compatibility
/// reasons, are closely identified with or tagged using an existing
/// primary language subtag.
///
/// The type [`LanguageExtension`] represents a list of
/// extended language.
///
/// # Grammar
///
/// ```abnf
/// ExtendedLangTag = 3ALPHA
/// ```
#[derive(RegularGrammar)]
#[grammar(sized(
	ExtendedLangTagBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
#[cfg_attr(feature = "serde", grammar(serde))]
pub struct ExtendedLangTag(str);

impl PartialEq for ExtendedLangTag {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for ExtendedLangTag {}

str_eq!(ExtendedLangTag);
str_eq!(ExtendedLangTagBuf);

impl PartialOrd for ExtendedLangTag {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for ExtendedLangTag {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for ExtendedLangTag {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
