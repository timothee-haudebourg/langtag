use std::convert::TryFrom;
use crate::Language;

pub struct InvalidGrandfatheredTag;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GrandfatheredTag {
	// regular
	EnGbOed,
	IAmi,
	IBnn,
	IDefault,
	IEnochian,
	IHak,
	IKlingon,
	ILux,
	IMingo,
	INavajo,
	IPwn,
	ITao,
	ITay,
	ITsu,
	SgnBeFr,
	SgnBeNl,
	SgnChDe,
	// irregular
	ArtLojban,
	CelGaulish,
	NoBok,
	NoNyn,
	ZhGuoyu,
	ZhHakka,
	ZhMin,
	ZhMinNan,
	ZhXiang
}

use GrandfatheredTag::*;

pub static GRANDFATHERED: [GrandfatheredTag; 26] = [
	EnGbOed,
	IAmi,
	IBnn,
	IDefault,
	IEnochian,
	IHak,
	IKlingon,
	ILux,
	IMingo,
	INavajo,
	IPwn,
	ITao,
	ITay,
	ITsu,
	SgnBeFr,
	SgnBeNl,
	SgnChDe,
	ArtLojban,
	CelGaulish,
	NoBok,
	NoNyn,
	ZhGuoyu,
	ZhHakka,
	ZhMin,
	ZhMinNan,
	ZhXiang
];

impl GrandfatheredTag {
	pub fn new<T: AsRef<[u8]>>(t: T) -> Result<GrandfatheredTag, T> {
		match Self::try_from(t.as_ref()) {
			Ok(tag) => Ok(tag),
			Err(InvalidGrandfatheredTag) => Err(t)
		}
	}

	pub fn language(&self) -> Option<&Language> {
		unsafe {
			match self {
				ArtLojban => Some(Language::new_unchecked(b"art")),
				CelGaulish => Some(Language::new_unchecked(b"cel")),
				NoBok => Some(Language::new_unchecked(b"no")),
				NoNyn => Some(Language::new_unchecked(b"no")),
				ZhGuoyu => Some(Language::new_unchecked(b"zh")),
				ZhHakka => Some(Language::new_unchecked(b"zh")),
				ZhMin => Some(Language::new_unchecked(b"zh")),
				ZhMinNan => Some(Language::new_unchecked(b"zh")),
				ZhXiang => Some(Language::new_unchecked(b"zh")),
				_ => None
			}
		}
	}

	pub fn as_bytes(&self) -> &[u8] {
		use GrandfatheredTag::*;
		match self {
			EnGbOed => b"en-GB-oed",
			IAmi => b"i-ami",
			IBnn => b"i-bnn",
			IDefault => b"i-default",
			IEnochian => b"i-enochian",
			IHak => b"i-hak",
			IKlingon => b"i-klingon",
			ILux => b"i-lux",
			IMingo => b"i-mingo",
			INavajo => b"i-navajo",
			IPwn => b"i-pwn",
			ITao => b"i-tao",
			ITay => b"i-tay",
			ITsu => b"i-tsu",
			SgnBeFr => b"sgn-BE-FR",
			SgnBeNl => b"sgn-BE-NL",
			SgnChDe => b"sgn-CH-DE",

			ArtLojban => b"art-lojban",
			CelGaulish => b"cel-gaulish",
			NoBok => b"no-bok",
			NoNyn => b"no-nyn",
			ZhGuoyu => b"zh-guoyu",
			ZhHakka => b"zh-hakka",
			ZhMin => b"zh-min",
			ZhMinNan => b"zh-min-nan",
			ZhXiang => b"zh-xiang"
		}
	}

	pub fn as_str(&self) -> &str {
		unsafe {
			std::str::from_utf8_unchecked(self.as_bytes())
		}
	}
}

impl<'a> TryFrom<&'a [u8]> for GrandfatheredTag {
	type Error = InvalidGrandfatheredTag;

	fn try_from(bytes: &'a [u8]) -> Result<GrandfatheredTag, InvalidGrandfatheredTag> {
		for tag in &GRANDFATHERED {
			if crate::case_insensitive_eq(tag.as_bytes(), bytes) {
				return Ok(*tag)
			}
		}

		Err(InvalidGrandfatheredTag)
	}
}