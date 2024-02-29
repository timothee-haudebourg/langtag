use std::hash::Hash;

use static_regular_grammar::RegularGrammar;

use crate::utils::{self, str_eq};

/// Primary language subtag.
///
/// The primary language subtag is the first subtag in a language tag.
///
/// # Grammar
///
/// ```abnf
/// PrimaryLanguage = 2*3ALPHA
/// ```
#[derive(RegularGrammar)]
#[grammar(
	file = "src/grammar.abnf",
	entry_point = "extlang",
	cache = "automata/extlang.aut.cbor"
)]
#[grammar(sized(
	PrimaryLanguageBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
#[cfg_attr(feature = "serde", grammar(serde))]
pub struct PrimaryLanguage(str);

impl PartialEq for PrimaryLanguage {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for PrimaryLanguage {}

str_eq!(PrimaryLanguage);
str_eq!(PrimaryLanguageBuf);

impl PartialOrd for PrimaryLanguage {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PrimaryLanguage {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for PrimaryLanguage {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
