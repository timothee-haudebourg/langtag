use std::hash::{
	Hash,
	Hasher
};
use crate::{
	parse,
	Language,
	Script,
	Region,
	Variant,
	Extension,
	PrivateUse
};

pub struct NormalLangTag<T> {
	p: parse::ParsedLangTag,
	data: T
}

impl<T: AsRef<[u8]>> NormalLangTag<T> {
	pub fn new(data: T) -> Result<NormalLangTag<T>, parse::Error> {
		let p = parse::langtag(data.as_ref(), 0)?;
		Ok(NormalLangTag {
			p,
			data
		})
	}

	pub fn language(&self) -> &Language {
		unsafe {
			Language::new_unchecked(&self.data.as_ref()[0..self.p.language_end])
		}
	}

	pub fn script(&self) -> Option<&Script> {
		if self.p.language_end < self.p.script_end {
			unsafe {
				Some(Script::new_unchecked(&self.data.as_ref()[self.p.language_end..self.p.script_end]))
			}
		} else {
			None
		}
	}

	pub fn region(&self) -> Option<&Region> {
		if self.p.script_end < self.p.region_end {
			unsafe {
				Some(Region::new_unchecked(&self.data.as_ref()[self.p.script_end..self.p.region_end]))
			}
		} else {
			None
		}
	}

	pub fn variant(&self) -> Option<&Variant> {
		if self.p.region_end < self.p.variant_end {
			unsafe {
				Some(Variant::new_unchecked(&self.data.as_ref()[self.p.region_end..self.p.variant_end]))
			}
		} else {
			None
		}
	}

	pub fn extension(&self) -> Option<&Extension> {
		if self.p.variant_end < self.p.extension_end {
			unsafe {
				Some(Extension::new_unchecked(&self.data.as_ref()[self.p.variant_end..self.p.extension_end]))
			}
		} else {
			None
		}
	}

	pub fn private_use(&self) -> Option<&PrivateUse> {
		if self.p.extension_end < self.p.privateuse_end {
			unsafe {
				Some(PrivateUse::new_unchecked(&self.data.as_ref()[self.p.extension_end..self.p.privateuse_end]))
			}
		} else {
			None
		}
	}
}

impl<T: AsRef<[u8]>, U: AsRef<[u8]>> PartialEq<NormalLangTag<U>> for NormalLangTag<T> {
	fn eq(&self, other: &NormalLangTag<U>) -> bool {
		crate::case_insensitive_eq(self.data.as_ref(), other.data.as_ref())
	}
}

impl<T: AsRef<[u8]>> Eq for NormalLangTag<T> {}

impl<T: AsRef<[u8]>> Hash for NormalLangTag<T> {
	fn hash<H: Hasher>(&self, h: &mut H) {
		crate::case_insensitive_hash(self.data.as_ref(), h)
	}
}

// impl<T: AsMut<Vec<u8>>> NormalLangTag<T> {
// 	pub fn set_language(&mut self, lang: &Language) {
// 		// ...
// 	}
// } 