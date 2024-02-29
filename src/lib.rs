//! This crate provides an implementation of *language tags* defined by
//! [RFC5646] ([BCP47]).
//!
//! [RFC5646]: <https://tools.ietf.org/html/rfc5646>
//! [BCP47]: <https://tools.ietf.org/html/bcp47>
//!
//! ## Usage
//!
//! You can easily parse new language from any string:
//! ```rust
//! use langtag::LangTag;
//!
//! fn main() -> Result<(), langtag::InvalidLangTag<&'static str>> {
//!   let tag = LangTag::new("fr-FR")?;
//!   assert_eq!(tag.language().unwrap().primary(), "fr");
//!   assert!(tag == "Fr-fr"); // comparison is case-insensitive.
//!   Ok(())
//! }
//! ```
//!
//! Note that [`LangTag::new`] does *not* copy the data it is given,
//! but only borrows it. The [`LangTagBuf`] type allows you to own the language
//! tag. Once parsed, you can explore every component of the language tag using
//! the provided functions.
//!
//! [`LangTag::new`]: crate::LangTag::new
//! [`LangTagBuf`]: crate::LangTagBuf
use std::hash::Hash;

use static_regular_grammar::RegularGrammar;

mod grandfathered;
mod normal;
mod private_use;
mod utils;

pub use grandfathered::*;
pub use normal::*;
pub use private_use::*;
use utils::str_eq;

/// Any language tag (normal, private use or grandfathered).
#[derive(RegularGrammar)]
#[grammar(file = "src/grammar.abnf", cache = "automata/langtag.aut.cbor")]
#[grammar(sized(
	LangTagBuf,
	derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)
))]
#[cfg_attr(feature = "serde", grammar(serde))]
pub struct LangTag(str);

impl LangTag {
	/// Returns the language subtags, if any.
	///
	/// Only normal language tags and regular grandfathered tags have language
	/// subtags.
	pub fn language(&self) -> Option<&Language> {
		match NormalLangTag::new(&self.0) {
			Ok(t) => Some(t.language()),
			Err(_) => match GrandfatheredLangTag::new(&self.0) {
				Ok(t) => t.language(),
				Err(_) => None,
			},
		}
	}

	/// Returns the script subtag, if any.
	pub fn script(&self) -> Option<&Script> {
		self.as_normal().and_then(NormalLangTag::script)
	}

	/// Returns the region subtag, if any.
	pub fn region(&self) -> Option<&Region> {
		self.as_normal().and_then(NormalLangTag::region)
	}

	/// Returns the variant subtags, if any.
	pub fn variants(&self) -> &Variants {
		self.as_normal()
			.map(NormalLangTag::variants)
			.unwrap_or(Variants::EMPTY)
	}

	/// Returns the extension subtags, if any.
	pub fn extensions(&self) -> &Extensions {
		self.as_normal()
			.map(NormalLangTag::extensions)
			.unwrap_or(Extensions::EMPTY)
	}

	/// Returns the private use subtag, if any.
	pub fn private_use(&self) -> Option<&PrivateUse> {
		self.as_normal().and_then(NormalLangTag::private_use)
	}

	/// Returns an iterator over the private use subtag subtags.
	pub fn private_use_subtags(&self) -> PrivateUseIter {
		self.private_use()
			.map(PrivateUse::iter)
			.unwrap_or(PrivateUseIter::empty())
	}

	/// Returns wether or not this language tag is a normal language tag.
	pub fn is_normal(&self) -> bool {
		self.as_normal().is_some()
	}

	/// Returns wether or not this language tag is a private use tag.
	pub fn is_private_use(&self) -> bool {
		self.as_private_use().is_some()
	}

	/// Returns wether or not this language tag is a grandfathered tag.
	pub fn is_grandfathered(&self) -> bool {
		self.as_grandfathered().is_some()
	}

	/// Returns this language tag as a normal tag, if it is one.
	pub fn as_normal(&self) -> Option<&NormalLangTag> {
		NormalLangTag::new(&self.0).ok()
	}

	/// Returns this language tag as a private use tag, if it is one.
	pub fn as_private_use(&self) -> Option<&PrivateUseLangTag> {
		PrivateUseLangTag::new(&self.0).ok()
	}

	/// Returns this language tag as a grandfathered tag, if it is one.
	pub fn as_grandfathered(&self) -> Option<GrandfatheredLangTag> {
		GrandfatheredLangTag::new(&self.0).ok()
	}

	/// Returns the kind of the tag (normal, private use or grandfathered).
	pub fn kind(&self) -> Kind {
		self.as_typed().kind()
	}

	/// Find out what kind of language tag `self` is.
	pub fn as_typed(&self) -> TypedLangTag {
		match NormalLangTag::new(&self.0) {
			Ok(t) => TypedLangTag::Normal(t),
			Err(_) => match PrivateUseLangTag::new(&self.0) {
				Ok(t) => TypedLangTag::PrivateUse(t),
				Err(_) => TypedLangTag::Grandfathered(GrandfatheredLangTag::new(&self.0).unwrap()),
			},
		}
	}
}

impl PartialEq for LangTag {
	fn eq(&self, other: &Self) -> bool {
		utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
	}
}

impl Eq for LangTag {}

str_eq!(LangTag);
str_eq!(LangTagBuf);

impl PartialOrd for LangTag {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for LangTag {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		utils::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
	}
}

impl Hash for LangTag {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		utils::case_insensitive_hash(self.as_bytes(), state)
	}
}

/// Language tag with type information (normal, private use or grandfathered).
pub enum TypedLangTag<'a> {
	Normal(&'a NormalLangTag),
	PrivateUse(&'a PrivateUseLangTag),
	Grandfathered(GrandfatheredLangTag),
}

impl<'a> TypedLangTag<'a> {
	/// Returns the language subtags, if any.
	///
	/// Only normal language tags and regular grandfathered tags have language
	/// subtags.
	pub fn language(&self) -> Option<&Language> {
		match self {
			Self::Normal(n) => Some(n.language()),
			Self::PrivateUse(_) => None,
			Self::Grandfathered(g) => g.language(),
		}
	}

	/// Returns this tag's kind (normal, private use or grandfathered).
	pub fn kind(&self) -> Kind {
		match self {
			Self::Normal(_) => Kind::Normal,
			Self::PrivateUse(_) => Kind::PrivateUse,
			Self::Grandfathered(_) => Kind::Grandfathered,
		}
	}

	/// Returns wether or not this language tag is a normal language tag.
	pub fn is_normal(&self) -> bool {
		matches!(self, Self::Normal(_))
	}

	/// Returns wether or not this language tag is a private use tag.
	pub fn is_private_use(&self) -> bool {
		matches!(self, Self::PrivateUse(_))
	}

	/// Returns wether or not this language tag is a grandfathered tag.
	pub fn is_grandfathered(&self) -> bool {
		matches!(self, Self::Grandfathered(_))
	}

	/// Returns this language tag as a normal tag, if it is one.
	pub fn as_normal(&self) -> Option<&'a NormalLangTag> {
		match self {
			Self::Normal(n) => Some(*n),
			_ => None,
		}
	}

	/// Returns this language tag as a private use tag, if it is one.
	pub fn as_private_use(&self) -> Option<&'a PrivateUseLangTag> {
		match self {
			Self::PrivateUse(p) => Some(*p),
			_ => None,
		}
	}

	/// Returns this language tag as a grandfathered tag, if it is one.
	pub fn as_grandfathered(&self) -> Option<GrandfatheredLangTag> {
		match self {
			Self::Grandfathered(g) => Some(*g),
			_ => None,
		}
	}
}

/// Language tag kind (normal, private use or grandfathered).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
	Normal,
	PrivateUse,
	Grandfathered,
}

impl Kind {
	/// Returns wether or not this is the normal language tag kind.
	pub fn is_normal(&self) -> bool {
		matches!(self, Self::Normal)
	}

	/// Returns wether or not this is the private use tag kind.
	pub fn is_private_use(&self) -> bool {
		matches!(self, Self::PrivateUse)
	}

	/// Returns wether or not this is the grandfathered tag kind.
	pub fn is_grandfathered(&self) -> bool {
		matches!(self, Self::Grandfathered)
	}
}
