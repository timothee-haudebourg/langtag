use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
	InvalidLangTag,
	InvalidExtendedLangTag,
	InvalidExtension,
	InvalidExtensionSubtag,
	InvalidExtensions,
	InvalidGrandfatheredTag,
	InvalidLangage,
	InvalidLangageExtension,
	InvalidPrimaryLangage,
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
			InvalidExtension => write!(f, "invalid language extension"),
			InvalidExtensionSubtag => write!(f, "invalid extended language subtag"),
			InvalidExtensions => write!(f, "invalid language extensions"),
			InvalidGrandfatheredTag => write!(f, "invalid grandfathered tag"),
			InvalidLangage => write!(f, "invalid language"),
			InvalidLangageExtension => write!(f, "invalid language extension subtag"),
			InvalidPrimaryLangage => write!(f, "invalid primary language"),
			InvalidPrivateUseSubtag => write!(f, "invalid private use subtag"),
			InvalidPrivateUseSubtags => write!(f, "invalid private use subtags"),
			InvalidRegion => write!(f, "invalid region subtag"),
			InvalidScript => write!(f, "invalid script subtag"),
			InvalidVariant => write!(f, "invalid variant subtag"),
			InvalidVariants => write!(f, "invalid variants")
		}
	}
}