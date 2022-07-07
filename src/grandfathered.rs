use crate::{Error, Language};
use std::convert::TryFrom;

/// Grandfathered tags.
///
/// Prior to [RFC 4646](https://tools.ietf.org/html/rfc4646),
/// whole language tags were registered according to
/// the rules in [RFC 1766](https://tools.ietf.org/html/rfc1766)
/// and/or [RFC 3066](https://tools.ietf.org/html/rfc3066).
/// All of these registered tags remain valid as language tags.
/// Many of these registered tags were made redundant by the advent of
/// either [RFC 4646](https://tools.ietf.org/html/rfc4646) or
/// [RFC 5646](https://tools.ietf.org/html/rfc5646).
/// The remainder of the previously registered tags are "grandfathered",
/// and are all veriants of this enum type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GrandfatheredTag {
	// irregular
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
	// regular
	ArtLojban,
	CelGaulish,
	NoBok,
	NoNyn,
	ZhGuoyu,
	ZhHakka,
	ZhMin,
	ZhMinNan,
	ZhXiang,
}

use GrandfatheredTag::*;

/// List of all grandfathered tags.
pub static GRANDFATHERED: [GrandfatheredTag; 26] = [
	EnGbOed, IAmi, IBnn, IDefault, IEnochian, IHak, IKlingon, ILux, IMingo, INavajo, IPwn, ITao,
	ITay, ITsu, SgnBeFr, SgnBeNl, SgnChDe, ArtLojban, CelGaulish, NoBok, NoNyn, ZhGuoyu, ZhHakka,
	ZhMin, ZhMinNan, ZhXiang,
];

impl GrandfatheredTag {
	/// Try to parse a grandfathered tag.
	#[inline]
	pub fn new<T: AsRef<[u8]>>(t: T) -> Result<GrandfatheredTag, T> {
		match Self::try_from(t.as_ref()) {
			Ok(tag) => Ok(tag),
			Err(_) => Err(t),
		}
	}

	/// Get the language the grandfathered tag if it is regular.
	#[inline]
	pub fn language(&self) -> Option<&Language> {
		unsafe {
			match self {
				ArtLojban => Some(Language::parse_unchecked(b"art")),
				CelGaulish => Some(Language::parse_unchecked(b"cel")),
				NoBok => Some(Language::parse_unchecked(b"no")),
				NoNyn => Some(Language::parse_unchecked(b"no")),
				ZhGuoyu => Some(Language::parse_unchecked(b"zh")),
				ZhHakka => Some(Language::parse_unchecked(b"zh")),
				ZhMin => Some(Language::parse_unchecked(b"zh")),
				ZhMinNan => Some(Language::parse_unchecked(b"zh")),
				ZhXiang => Some(Language::parse_unchecked(b"zh")),
				_ => None,
			}
		}
	}

	/// Returns the bytes representation of the tag.
	#[inline]
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
			ZhXiang => b"zh-xiang",
		}
	}

	/// Returns the string representation of the tag.
	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
	}
}

impl<'a> TryFrom<&'a [u8]> for GrandfatheredTag {
	type Error = Error;

	#[inline]
	fn try_from(bytes: &'a [u8]) -> Result<GrandfatheredTag, Error> {
		for tag in &GRANDFATHERED {
			if crate::case_insensitive_eq(tag.as_bytes(), bytes) {
				return Ok(*tag);
			}
		}

		Err(Error::InvalidGrandfatheredTag)
	}
}
