use std::{
	fmt,
	hash::{
		Hash,
		Hasher
	}
};
use crate::{
	Error,
	PrivateUseSubtags
};

pub struct PrivateUseTag<T: ?Sized = [u8]> {
	data: T
}

impl<T: AsRef<[u8]>> PrivateUseTag<T> {
	#[inline]
	pub fn new(t: T) -> Result<PrivateUseTag<T>, T> {
		let bytes = t.as_ref();
		if bytes.len() > 0 && crate::parse::privateuse(bytes, 0) == bytes.len() {
			Ok(PrivateUseTag {
				data: t
			})
		} else {
			Err(t)
		}
	}
}

impl PrivateUseTag {
	pub fn parse<'a, T: AsRef<[u8]> + ?Sized>(bytes: &T) -> Result<&'a PrivateUseTag, Error> {
		let bytes = bytes.as_ref();
		if bytes.len() > 0 && crate::parse::privateuse(bytes, 0) == bytes.len() {
			Ok(unsafe {
				&*(bytes as *const [u8] as *const PrivateUseTag)
			})
		} else {
			Err(Error::InvalidPrivateUseSubtags)
		}
	}
}

impl<T: AsRef<[u8]> + ?Sized> PrivateUseTag<T> {
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
	pub fn as_ref(&self) -> &PrivateUseSubtags {
		unsafe {
			PrivateUseSubtags::parse_unchecked(self.as_bytes())
		}
	}
}

impl<T: AsRef<[u8]> + ?Sized> AsRef<[u8]> for PrivateUseTag<T> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<T: AsRef<[u8]> + ?Sized, U: AsRef<[u8]> + ?Sized> PartialEq<U> for PrivateUseTag<T> {
	#[inline]
	fn eq(&self, other: &U) -> bool {
		crate::case_insensitive_eq(self.data.as_ref(), other.as_ref())
	}
}

impl<T: AsRef<[u8]> + ?Sized> Eq for PrivateUseTag<T> {}

impl<T: AsRef<[u8]> + ?Sized> Hash for PrivateUseTag<T> {
	#[inline]
	fn hash<H: Hasher>(&self, h: &mut H) {
		crate::case_insensitive_hash(self.data.as_ref(), h)
	}
}

impl<T: AsRef<[u8]> + ?Sized> fmt::Display for PrivateUseTag<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Display::fmt(self.as_str(), f)
	}
}

impl<T: AsRef<[u8]> + ?Sized> fmt::Debug for PrivateUseTag<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self.as_str(), f)
	}
}