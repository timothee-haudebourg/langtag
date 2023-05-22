use crate::LanguageTag;

/// Type that can be borrowed as a [`LanguageTag`] reference.
///
/// This is the equivalent of `AsRef` for `LanguageTag`.
pub trait AsLanguageTag<T: ?Sized = [u8]> {
	fn as_language_tag(&self) -> LanguageTag<T>;
}
