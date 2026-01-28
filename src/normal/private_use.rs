use core::hash::Hash;

use crate::utils;

/// Private use.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Privateuse)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(PrivateUseBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct PrivateUse(str);

impl PrivateUse {
	pub fn iter(&self) -> PrivateUseIter<'_> {
		PrivateUseIter::new(&self.0)
	}
}

impl PartialEq for PrivateUse {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for PrivateUse {}

impl PartialOrd for PrivateUse {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PrivateUse {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for PrivateUse {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
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
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::PrivateuseSubtag)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(PrivateUseSubtagBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct PrivateUseSubtag(str);

impl PartialEq for PrivateUseSubtag {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for PrivateUseSubtag {}

impl PartialOrd for PrivateUseSubtag {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PrivateUseSubtag {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for PrivateUseSubtag {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}
