use core::hash::Hash;

use crate::utils;

mod extlang;
mod primary;

pub use extlang::*;
pub use primary::*;

/// Primary and extended language subtags.
///
/// This type represents the primary language subtag (first subtag in a
/// language tag) and the extended language subtags associated with it.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Language)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(LanguageBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct Language(str);

impl Language {
	/// Return the primary language subtag.
	///
	/// The primary language subtag is the first subtag in a language tag.
	#[inline]
	pub fn primary(&self) -> &PrimaryLanguage {
		let primary = match self.0.split_once('-') {
			Some((primary, _)) => primary,
			None => &self.0,
		};

		unsafe { PrimaryLanguage::new_unchecked(primary) }
	}

	/// Return the extended language subtags.
	///
	/// Extended language subtags are used to identify certain specially
	/// selected languages that, for various historical and compatibility
	/// reasons, are closely identified with or tagged using an existing
	/// primary language subtag.
	///
	/// Extended language subtags are only present when the primary
	/// language subtag length is 2 or 3.
	#[inline]
	pub fn extension(&self) -> Option<&LanguageExtension> {
		self.0
			.split_once('-')
			.map(|(_, ext)| unsafe { LanguageExtension::new_unchecked(ext) })
	}

	/// Return an iterator to the extended language subtags.
	#[inline]
	pub fn extension_subtags(&self) -> LanguageExtensionIter<'_> {
		self.extension()
			.map(LanguageExtension::iter)
			.unwrap_or_default()
	}
}

impl PartialEq for Language {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for Language {}

impl PartialOrd for Language {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Language {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for Language {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
