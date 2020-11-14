use std::hash::{
	Hash,
	Hasher
};
use crate::PrivateUse;

pub struct PrivateUseTag<T> {
	data: T
}

impl<T: AsRef<[u8]>> PrivateUseTag<T> {
	#[inline]
	pub fn new(t: T) -> Result<PrivateUseTag<T>, T> {
		let bytes = t.as_ref();
		if crate::parse::privateuse(bytes, 0) == bytes.len() {
			Ok(PrivateUseTag {
				data: t
			})
		} else {
			Err(t)
		}
	}

	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.data.as_ref()
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe {
			std::str::from_utf8_unchecked(self.as_bytes())
		}
	}

	#[inline]
	pub fn as_ref(&self) -> &PrivateUse {
		unsafe {
			PrivateUse::new_unchecked(self.as_bytes())
		}
	}
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for PrivateUseTag<T> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]>> PartialEq<U> for PrivateUseTag<T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		crate::case_insensitive_eq(self.data.as_ref(), other.as_ref())
	}
}

impl<T: AsRef<[u8]>> Eq for PrivateUseTag<T> {}

impl<T: AsRef<[u8]>> Hash for PrivateUseTag<T> {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		crate::case_insensitive_hash(self.data.as_ref(), h)
	}
}