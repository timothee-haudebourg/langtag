use core::hash::Hash;

use crate::utils;

/// Script subtag.
///
/// Script subtags are used to indicate the script or writing system
/// variations that distinguish the written forms of a language or its
/// dialects.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Script)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(ScriptBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct Script(str);

impl PartialEq for Script {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for Script {}

impl PartialOrd for Script {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Script {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for Script {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
