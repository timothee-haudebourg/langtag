use std::{
	hash::{
		Hash,
		Hasher
	},
	cmp::Ordering
};

mod parse;
pub mod privateusetag;
pub mod langtag;
mod grandfathered;

pub use grandfathered::*;

pub enum RawLangTag<T> {
	Normal(langtag::NormalLangTag<T>),
	PrivateUse(privateusetag::PrivateUseTag<T>),
	Grandfathered(GrandfatheredTag)
}

impl<T: AsRef<[u8]>> RawLangTag<T> {
	pub fn new(t: T) -> Result<RawLangTag<T>, parse::Error> {
		match GrandfatheredTag::new(t) {
			Ok(tag) => Ok(RawLangTag::Grandfathered(tag)),
			Err(t) => match privateusetag::PrivateUseTag::new(t) {
				Ok(tag) => Ok(RawLangTag::PrivateUse(tag)),
				Err(t) => {
					Ok(RawLangTag::Normal(langtag::NormalLangTag::new(t)?))
				}
			}
		}
	}
}

pub type NormalLangTag<'a> = langtag::NormalLangTag<&'a [u8]>;
pub type NormalLangTagBuf = langtag::NormalLangTag<Vec<u8>>;

pub type PrivateUseTag<'a> = privateusetag::PrivateUseTag<&'a [u8]>;
pub type PrivateUseTagBuf = privateusetag::PrivateUseTag<Vec<u8>>;

pub type LangTag<'a> = RawLangTag<&'a [u8]>;
pub type LangTagBuf = RawLangTag<Vec<u8>>;

pub(crate) fn into_smallcase(c: u8) -> u8 {
	if c >= b'A' && c <= b'Z' {
		c - 0x20
	} else {
		c
	}
}

pub(crate) fn case_insensitive_eq(a: &[u8], b: &[u8]) -> bool {
	if a.len() == b.len() {
		for i in 0..a.len() {
			if into_smallcase(a[i]) != into_smallcase(b[i]) {
				return false
			}
		}

		true
	} else {
		false
	}
}

pub(crate) fn case_insensitive_hash<H: Hasher>(bytes: &[u8], hasher: &mut H) {
	for b in bytes {
		into_smallcase(*b).hash(hasher)
	}
}

pub(crate) fn case_insensitive_cmp(a: &[u8], b: &[u8]) -> Ordering {
	let mut i = 0;

	loop {
		if a.len() <= i {
			if b.len() <= i {
				return Ordering::Equal
			}

			return Ordering::Greater
		} else if b.len() <= i {
			return Ordering::Less
		} else {
			match into_smallcase(a[i]).cmp(&into_smallcase(b[i])) {
				Ordering::Equal => {
					i += 1
				},
				ord => return ord
			}
		}
	}
}

macro_rules! component {
	($parser:ident, $id:ident, $err:ident) => {
		mod $parser {
			use std::{
				hash::{
					Hash,
					Hasher
				},
				convert::TryFrom,
				cmp::{
					PartialOrd,
					Ord,
					Ordering
				},
				ops::Deref
			};

			pub struct $err;

			pub struct $id {
				data: [u8]
			}

			impl $id {
				#[inline]
				pub fn new<B: AsRef<[u8]> + ?Sized>(bytes: &B) -> Result<&$id, $err> {
					let bytes = bytes.as_ref();
					
					if crate::parse::$parser(bytes.as_ref(), 0) == bytes.len() {
						Ok(unsafe { Self::new_unchecked(bytes) })
					} else {
						Err($err)
					}
				}

				#[inline]
				pub unsafe fn new_unchecked<B: AsRef<[u8]> + ?Sized>(bytes: &B) -> &$id {
					&*(bytes.as_ref() as *const [u8] as *const $id)
				}

				#[inline]
				pub fn as_bytes(&self) -> &[u8] {
					&self.data
				}

				#[inline]
				pub fn as_str(&self) -> &str {
					unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
				}
			}

			impl<'a> TryFrom<&'a str> for &'a $id {
				type Error = $err;

				#[inline]
				fn try_from(b: &'a str) -> Result<&'a $id, $err> {
					$id::new(b)
				}
			}

			impl<'a> TryFrom<&'a [u8]> for &'a $id {
				type Error = $err;

				#[inline]
				fn try_from(b: &'a [u8]) -> Result<&'a $id, $err> {
					$id::new(b)
				}
			}

			impl AsRef<[u8]> for $id {
				#[inline]
				fn as_ref(&self) -> &[u8] {
					self.as_bytes()
				}
			}

			impl AsRef<str> for $id {
				#[inline]
				fn as_ref(&self) -> &str {
					self.as_str()
				}
			}

			impl Deref for $id {
				type Target = [u8];

				#[inline]
				fn deref(&self) -> &[u8] {
					self.as_bytes()
				}
			}

			impl<T: AsRef<[u8]> + ?Sized> PartialOrd<T> for $id {
				#[inline]
				fn partial_cmp(&self, other: &T) -> Option<Ordering> {
					Some(crate::case_insensitive_cmp(self.as_bytes(), other.as_ref()))
				}
			}

			impl Ord for $id {
				#[inline]
				fn cmp(&self, other: &$id) -> Ordering {
					crate::case_insensitive_cmp(self.as_bytes(), other.as_bytes())
				}
			}

			impl<T: AsRef<[u8]> + ?Sized> PartialEq<T> for $id {
				#[inline]
				fn eq(&self, other: &T) -> bool {
					crate::case_insensitive_eq(self.as_bytes(), other.as_ref())
				}
			}

			impl Eq for $id {}

			impl Hash for $id {
				#[inline]
				fn hash<H: Hasher>(&self, h: &mut H) {
					crate::case_insensitive_hash(&self.data, h)
				}
			}
		}

		pub use $parser::*;
	};
}

component!(language, Language, InvalidLangage);
component!(script, Script, InvalidScript);
component!(region, Region, InvalidRegion);
component!(variant, Variant, InvalidVariant);
component!(extension, Extension, InvalidExtension);
component!(privateuse, PrivateUse, InvalidPrivateUse);