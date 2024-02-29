use std::{
	cmp::Ordering,
	hash::{Hash, Hasher},
};

macro_rules! str_eq {
	($ty:ident) => {
		impl PartialEq<str> for $ty {
			fn eq(&self, other: &str) -> bool {
				crate::utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}

		impl<'a> PartialEq<&'a str> for $ty {
			fn eq(&self, other: &&'a str) -> bool {
				crate::utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}

		impl PartialEq<String> for $ty {
			fn eq(&self, other: &String) -> bool {
				crate::utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}

		impl PartialEq<$ty> for str {
			fn eq(&self, other: &$ty) -> bool {
				crate::utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}

		impl PartialEq<$ty> for String {
			fn eq(&self, other: &$ty) -> bool {
				crate::utils::case_insensitive_eq(self.as_bytes(), other.as_bytes())
			}
		}
	};
}

pub(crate) use str_eq;

pub fn into_smallcase(c: u8) -> u8 {
	if c.is_ascii_uppercase() {
		c + 0x20
	} else {
		c
	}
}

pub fn case_insensitive_eq(a: &[u8], b: &[u8]) -> bool {
	if a.len() == b.len() {
		for i in 0..a.len() {
			if into_smallcase(a[i]) != into_smallcase(b[i]) {
				return false;
			}
		}

		true
	} else {
		false
	}
}

pub fn case_insensitive_hash<H: Hasher>(bytes: &[u8], hasher: &mut H) {
	for b in bytes {
		into_smallcase(*b).hash(hasher)
	}
}

pub fn case_insensitive_cmp(a: &[u8], b: &[u8]) -> Ordering {
	let mut i = 0;

	loop {
		if a.len() <= i {
			if b.len() <= i {
				return Ordering::Equal;
			}

			return Ordering::Greater;
		} else if b.len() <= i {
			return Ordering::Less;
		} else {
			match into_smallcase(a[i]).cmp(&into_smallcase(b[i])) {
				Ordering::Equal => i += 1,
				ord => return ord,
			}
		}
	}
}
