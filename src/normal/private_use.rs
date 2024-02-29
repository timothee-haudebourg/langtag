use crate::utils::{self, str_eq};
use static_regular_grammar::RegularGrammar;
use std::hash::Hash;

/// Private use.
#[derive(RegularGrammar)]
#[grammar(
	file = "src/grammar.abnf",
	entry_point = "privateuse",
	cache = "automata/private-use.aut.cbor"
)]
#[grammar(sized(
	PrivateUseBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
#[cfg_attr(feature = "serde", grammar(serde))]
pub struct PrivateUse(str);

impl PrivateUse {
	pub fn iter(&self) -> PrivateUseIter {
		PrivateUseIter::new(&self.0)
	}
}

impl PartialEq for PrivateUse {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for PrivateUse {}

str_eq!(PrivateUse);
str_eq!(PrivateUseBuf);

impl PartialOrd for PrivateUse {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PrivateUse {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for PrivateUse {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}

#[derive(Default)]
pub struct PrivateUseIter<'a> {
	data: &'a str,
	offset: usize,
}

impl<'a> PrivateUseIter<'a> {
	fn new(data: &'a str) -> Self {
		Self {
			data,
			offset: 2, // start after the singleton
		}
	}

	pub fn empty() -> Self {
		Self::default()
	}
}

impl<'a> Iterator for PrivateUseIter<'a> {
	type Item = &'a PrivateUseSubtag;

	fn next(&mut self) -> Option<Self::Item> {
		if self.offset < self.data.len() {
			let end = super::find_segment_end(self.data, self.offset);
			let item = unsafe { &PrivateUseSubtag::new_unchecked(&self.data[self.offset..end]) };
			self.offset = end + 1;
			Some(item)
		} else {
			None
		}
	}
}

/// Private use subtag.
///
/// # Grammar
///
/// ```abnf
/// PrivateUseSubtag = 1*8alphanum
///
/// alphanum         = (ALPHA / DIGIT) ; letters and numbers
/// ```
#[derive(RegularGrammar)]
#[grammar(sized(
	PrivateUseSubtagBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
#[cfg_attr(feature = "serde", grammar(serde))]
pub struct PrivateUseSubtag(str);

impl PartialEq for PrivateUseSubtag {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for PrivateUseSubtag {}

str_eq!(PrivateUseSubtag);
str_eq!(PrivateUseSubtagBuf);

impl PartialOrd for PrivateUseSubtag {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PrivateUseSubtag {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for PrivateUseSubtag {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
