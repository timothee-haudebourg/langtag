use core::hash::Hash;

use crate::utils;

/// Primary language subtag.
///
/// The primary language subtag is the first subtag in a language tag.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Primary)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(PrimaryLanguageBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct PrimaryLanguage(str);

impl PartialEq for PrimaryLanguage {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for PrimaryLanguage {}

impl PartialOrd for PrimaryLanguage {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PrimaryLanguage {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for PrimaryLanguage {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
