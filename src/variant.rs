use std::{
	cmp::Ordering,
	ops::Deref,
	hash::{
		Hash,
		Hasher
	},
	fmt,
	convert::TryFrom
};
use crate::{
	parse,
	Error
};

component! {
	/// Single variant subtag.
	/// 
	/// Variant subtags are used to indicate additional, well-recognized
	/// variations that define a language or its dialects that are not
	/// covered by other available subtags.
	variant, false, Variant, InvalidVariant
}

component! {
	/// List of variant subtags.
	/// 
	/// Represents a list of variant subtags separated by a `-` character
	/// as found in a language tag.
	variants, true, Variants, InvalidVariants
}

iterator! {
	/// Variant subtags iterator.
	Variants, VariantsIter, Variant, 0
}

/// Mutable reference to the variants of a language tag.
pub struct VariantsMut<'a> {
	/// Language tag buffer.
	pub(crate) buffer: &'a mut Vec<u8>,

	/// Language tag parsing data.
	pub(crate) p: &'a mut parse::ParsedLangTag
}

impl<'a> VariantsMut<'a> {
	/// Checks if the list of variants is empty.
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.p.variant_end <= self.p.region_end+1
	}

	/// Returns the first variant subtag of the list (if any).
	#[inline]
	pub fn first(&self) -> Option<&Variant> {
		if self.is_empty() {
			None
		} else {
			let mut i = self.p.region_end+1;

			while i < self.p.variant_end && self.buffer[i] != b'-' {
				i += 1
			}

			unsafe {
				Some(Variant::parse_unchecked(&self.buffer[self.p.region_end+1..i]))
			}
		}
	}

	/// Returns the last variant subtag of the list (if any).
	#[inline]
	pub fn last(&self) -> Option<&Variant> {
		if self.is_empty() {
			None
		} else {
			let mut i = self.p.variant_end-1;

			while i > self.p.region_end+2 && self.buffer[i-1] != b'-' {
				i -= 1
			}

			unsafe {
				Some(Variant::parse_unchecked(&self.buffer[i..self.p.variant_end]))
			}
		}
	}

	/// Add a new variant subtag at the end.
	#[inline]
	pub fn push(&mut self, variant: &Variant) {
		let bytes = variant.as_bytes();

		let mut i = self.p.variant_end;
		crate::replace(self.buffer, i..i, b"-");
		i += 1;
		crate::replace(self.buffer, i..i, bytes);

		let len = bytes.len() + 1;
		self.p.variant_end += len;
		self.p.extension_end += len;
		self.p.privateuse_end += len;
	}

	/// Removes and return the last variant subtag of the list (if any).
	#[inline]
	pub fn pop(&mut self) -> Option<Variant<Vec<u8>>> {
		match self.last() {
			Some(last) => {
				let mut new_end = self.p.variant_end - last.len();

				let copy = unsafe {
					Variant::parse_copy_unchecked(&self.buffer[new_end..self.p.variant_end])
				};

				if new_end > self.p.region_end {
					new_end -= 1
				}

				crate::replace(self.buffer, new_end..self.p.variant_end, b"");

				let len = self.p.variant_end - new_end;
				self.p.variant_end -= len;
				self.p.extension_end -= len;
				self.p.privateuse_end -= len;

				Some(copy)
			},
			None => None
		}
	}
}