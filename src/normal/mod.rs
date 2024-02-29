use crate::utils::{self, str_eq};
use static_regular_grammar::RegularGrammar;
use std::{hash::Hash, ops::Range};

mod language;
pub use language::*;

mod script;
pub use script::*;

mod region;
pub use region::*;

mod variant;
pub use variant::*;

mod extension;
pub use extension::*;

mod private_use;
pub use private_use::*;

/// Normal language tag.
#[derive(RegularGrammar)]
#[grammar(
	file = "src/grammar.abnf",
	entry_point = "langtag",
	cache = "automata/normal.aut.cbor"
)]
#[grammar(sized(
	NormalLangTagBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
#[cfg_attr(feature = "serde", grammar(serde))]
pub struct NormalLangTag(str);

impl NormalLangTag {
	fn language_end(&self) -> usize {
		find_list_end(&self.0, 0, |i, segment| {
			i == 0 || ExtendedLangTag::new(segment).is_ok()
		})
	}

	/// Returns the language subtags.
	pub fn language(&self) -> &Language {
		unsafe { Language::new_unchecked(&self.0[..self.language_end()]) }
	}

	fn script_range(&self) -> Result<Range<usize>, usize> {
		let language_end = self.language_end();
		eprintln!("language_end = {language_end}");
		let offset = language_end + 1;
		let end = find_list_end(&self.0, offset, |i, segment| {
			i == offset && Script::new(segment).is_ok()
		});
		if end != offset {
			Ok(offset..end)
		} else {
			Err(language_end)
		}
	}

	/// Returns the script subtag, if any.
	pub fn script(&self) -> Option<&Script> {
		self.script_range()
			.ok()
			.map(|range| unsafe { Script::new_unchecked(&self.0[range]) })
	}

	fn region_range(&self) -> Result<Range<usize>, usize> {
		let script_end = match self.script_range() {
			Ok(range) => range.end,
			Err(i) => i,
		};
		eprintln!("script_end = {script_end}");

		let offset = script_end + 1;
		let end = find_list_end(&self.0, offset, |i, segment| {
			i == offset && Region::new(segment).is_ok()
		});
		if end != offset {
			Ok(offset..end)
		} else {
			Err(script_end)
		}
	}

	/// Returns the region subtag, if any.
	pub fn region(&self) -> Option<&Region> {
		self.region_range()
			.ok()
			.map(|range| unsafe { Region::new_unchecked(&self.0[range]) })
	}

	fn variants_range(&self) -> Range<usize> {
		let region_end = match self.region_range() {
			Ok(range) => range.end,
			Err(i) => i,
		};
		eprintln!("region_end = {region_end}");

		let offset = region_end + 1;
		let end = find_list_end(&self.0, offset, |_, segment| Variant::new(segment).is_ok());
		if end == offset {
			region_end..region_end
		} else {
			offset..end
		}
	}

	/// Returns the variant subtags.
	pub fn variants(&self) -> &Variants {
		unsafe { Variants::new_unchecked(&self.0[self.variants_range()]) }
	}

	fn extensions_range(&self) -> Range<usize> {
		let variants_end = self.variants_range().end;
		eprintln!("variants_end = {variants_end}");

		let offset = variants_end + 1;
		let end = find_list_end(&self.0, offset, |_, segment| {
			Singleton::from_string(segment).is_ok() || ExtensionSubtag::new(segment).is_ok()
		});

		if end == offset {
			variants_end..variants_end
		} else {
			offset..end
		}
	}

	/// Returns the extension subtags.
	pub fn extensions(&self) -> &Extensions {
		eprintln!("looking for extensions in {}", &self.0);
		unsafe { Extensions::new_unchecked(&self.0[self.extensions_range()]) }
	}

	fn private_use_offset(&self) -> Option<usize> {
		let variants_end = self.variants_range().end;

		if variants_end < self.0.len() {
			Some(variants_end + 1)
		} else {
			None
		}
	}

	/// Returns the private use subtags.
	pub fn private_use(&self) -> Option<&PrivateUse> {
		self.private_use_offset()
			.map(|i| unsafe { PrivateUse::new_unchecked(&self.0[i..]) })
	}

	pub fn private_use_subtags(&self) -> PrivateUseIter {
		match self.private_use() {
			Some(p) => p.iter(),
			None => PrivateUseIter::empty(),
		}
	}
}

impl PartialEq for NormalLangTag {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for NormalLangTag {}

str_eq!(NormalLangTag);
str_eq!(NormalLangTagBuf);

impl PartialOrd for NormalLangTag {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for NormalLangTag {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for NormalLangTag {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}

/// Find the end of a subtag list using the `f` function to determine which
/// subtag is part of the list.
fn find_list_end(string: &str, mut offset: usize, mut f: impl FnMut(usize, &str) -> bool) -> usize {
	let bytes = string.as_bytes();
	let mut i = offset;
	let mut end = i;
	while i < bytes.len() {
		if bytes[i] == b'-' {
			let subtag = &string[offset..i];
			if f(offset, subtag) {
				end = i;
				offset = i + 1;
			} else {
				return end;
			}
		}

		i += 1
	}

	let subtag = &string[offset..];
	if f(offset, subtag) {
		string.len()
	} else {
		end
	}
}

fn find_segment_end(string: &str, offset: usize) -> usize {
	let bytes = string.as_bytes();
	let mut i = offset;
	while i < bytes.len() {
		if bytes[i] == b'-' {
			return i;
		}

		i += 1
	}

	i
}

fn find_segment_start(string: &str, end: usize) -> usize {
	if end > 0 {
		let bytes = string.as_bytes();
		let mut i = end - 1;
		loop {
			if bytes[i] == b'-' {
				break i + 1;
			}

			if i > 0 {
				i -= 1
			} else {
				break 0;
			}
		}
	} else {
		0
	}
}
