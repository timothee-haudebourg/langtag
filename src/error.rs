use std::fmt;

/// Parsing errors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
	InvalidLangTag,
	InvalidExtendedLangTag,
	InvalidSingleton(u8),
	InvalidCharSingleton(char),
	InvalidExtension,
	InvalidExtensionSubtag,
	InvalidExtensions,
	InvalidGrandfatheredTag,
	InvalidLanguage,
	InvalidLanguageExtension,
	InvalidPrimaryLanguage,
	InvalidPrivateUseSubtag,
	InvalidPrivateUseSubtags,
	InvalidRegion,
	InvalidScript,
	InvalidVariant,
	InvalidVariants
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Error::*;
		match self {
			InvalidLangTag => write!(f, "invalid lang tag"),
			InvalidExtendedLangTag => write!(f, "invalid extended language"),
			InvalidSingleton(b) => write!(f, "invalid singleton `0x{:x}`", b),
			InvalidCharSingleton(c) => write!(f, "invalid singleton `{}`", c),
			InvalidExtension => write!(f, "invalid language extension"),
			InvalidExtensionSubtag => write!(f, "invalid extended language subtag"),
			InvalidExtensions => write!(f, "invalid language extensions"),
			InvalidGrandfatheredTag => write!(f, "invalid grandfathered tag"),
			InvalidLanguage => write!(f, "invalid language"),
			InvalidLanguageExtension => write!(f, "invalid language extension subtag"),
			InvalidPrimaryLanguage => write!(f, "invalid primary language"),
			InvalidPrivateUseSubtag => write!(f, "invalid private use subtag"),
			InvalidPrivateUseSubtags => write!(f, "invalid private use subtags"),
			InvalidRegion => write!(f, "invalid region subtag"),
			InvalidScript => write!(f, "invalid script subtag"),
			InvalidVariant => write!(f, "invalid variant subtag"),
			InvalidVariants => write!(f, "invalid variants")
		}
	}
}