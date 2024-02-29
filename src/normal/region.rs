use std::hash::Hash;

use static_regular_grammar::RegularGrammar;

use crate::utils::{self, str_eq};

/// Region subtag.
///
/// Region subtags are used to indicate linguistic variations associated
/// with or appropriate to a specific country, territory, or region.
/// Typically, a region subtag is used to indicate variations such as
/// regional dialects or usage, or region-specific spelling conventions.
/// It can also be used to indicate that content is expressed in a way
/// that is appropriate for use throughout a region, for instance,
/// Spanish content tailored to be useful throughout Latin America.
#[derive(RegularGrammar)]
#[grammar(file = "src/grammar.abnf", entry_point = "region")]
#[grammar(sized(
	RegionBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
#[cfg_attr(feature = "serde", grammar(serde))]
pub struct Region(str);

impl PartialEq for Region {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for Region {}

str_eq!(Region);
str_eq!(RegionBuf);

impl PartialOrd for Region {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Region {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for Region {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
