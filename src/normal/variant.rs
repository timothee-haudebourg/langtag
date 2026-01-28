use core::hash::Hash;

use crate::utils;

/// Single variant subtag.
///
/// Variant subtags are used to indicate additional, well-recognized
/// variations that define a language or its dialects that are not
/// covered by other available subtags.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Variant)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(VariantBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct Variant(str);

impl PartialEq for Variant {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for Variant {}

impl PartialOrd for Variant {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Variant {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for Variant {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}

/// List of variant subtags.
///
/// Represents a list of variant subtags separated by a `-` character
/// as found in a language tag.
#[derive(static_automata::Validate, str_newtype::StrNewType)]
#[automaton(crate::grammar::Variants)]
#[newtype(
	no_deref,
	ord([u8], &[u8], str, &str)
)]
#[cfg_attr(
	feature = "std",
	newtype(ord(Vec<u8>, String), owned(VariantsBuf, derive(PartialEq, Eq, PartialOrd, Ord, Hash)))
)]
#[cfg_attr(feature = "serde", newtype(serde))]
pub struct Variants(str);

impl Variants {
	pub const EMPTY: &Self = unsafe { Self::new_unchecked("") };

	pub fn iter(&self) -> VariantsIter<'_> {
		VariantsIter::new(&self.0)
	}

	pub fn first(&self) -> Option<&Variant> {
		self.iter().next()
	}

	pub fn last(&self) -> Option<&Variant> {
		self.iter().next_back()
	}
}

impl PartialEq for Variants {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for Variants {}

impl PartialOrd for Variants {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Variants {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for Variants {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}

#[derive(Default)]
pub struct VariantsIter<'a> {
	data: &'a str,
	offset: usize,
	end: usize,
}

impl<'a> VariantsIter<'a> {
	fn new(data: &'a str) -> Self {
		Self {
			data,
			offset: 0,
			end: data.len(),
		}
	}

	pub fn empty() -> Self {
		Self::default()
	}
}

impl<'a> Iterator for VariantsIter<'a> {
	type Item = &'a Variant;

	fn next(&mut self) -> Option<Self::Item> {
		if self.offset < self.end {
			let end = super::find_segment_end(self.data, self.offset);
			let item = unsafe { Variant::new_unchecked(&self.data[self.offset..end]) };
			self.offset = end + 1;
			Some(item)
		} else {
			None
		}
	}
}

impl<'a> DoubleEndedIterator for VariantsIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.end > self.offset {
			let start = super::find_segment_start(self.data, self.end);
			let item = unsafe { Variant::new_unchecked(&self.data[start..self.end]) };

			self.end = if start > 0 { start - 1 } else { 0 };

			Some(item)
		} else {
			None
		}
	}
}
