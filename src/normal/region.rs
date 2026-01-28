use core::hash::Hash;

use crate::utils;

/// Region subtag.
///
/// Region subtags are used to indicate linguistic variations associated
/// with or appropriate to a specific country, territory, or region.
/// Typically, a region subtag is used to indicate variations such as
/// regional dialects or usage, or region-specific spelling conventions.
/// It can also be used to indicate that content is expressed in a way
/// that is appropriate for use throughout a region, for instance,
/// Spanish content tailored to be useful throughout Latin America.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Region)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(RegionBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct Region(str);

impl PartialEq for Region {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for Region {}

impl PartialOrd for Region {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Region {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for Region {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
