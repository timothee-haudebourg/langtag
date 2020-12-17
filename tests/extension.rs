use std::convert::TryInto;
use langtag::LanguageTag;

#[test]
pub fn extensions_eq() {
	let tag = LanguageTag::parse("fr-a-ext1-ext2-b-ext3-ext4-c-ext5-ext6").unwrap();
	assert_eq!(tag.extensions(), "a-ext1-ext2-b-ext3-ext4-c-ext5-ext6");
}

#[test]
pub fn extensions_iter() {
	let tag = LanguageTag::parse("fr-a-ext1-ext2-b-ext3-ext4-c-ext5-ext6").unwrap();
	let mut it = tag.extensions().iter();
	assert_eq!(it.next().unwrap(), "a-ext1-ext2");
	assert_eq!(it.next().unwrap(), "b-ext3-ext4");
	assert_eq!(it.next().unwrap(), "c-ext5-ext6");
	assert_eq!(it.next(), None);
}

#[test]
pub fn extensions_iter_extension() {
	let tag = LanguageTag::parse("fr-a-ext1-b-ext2-a-ext3-ext4-a-ext5-ext6").unwrap();
	let mut it = tag.extensions().iter_extension('a'.try_into().unwrap());
	assert_eq!(it.next().unwrap(), "ext1");
	assert_eq!(it.next().unwrap(), "ext3");
	assert_eq!(it.next().unwrap(), "ext4");
	assert_eq!(it.next().unwrap(), "ext5");
	assert_eq!(it.next().unwrap(), "ext6");
	assert_eq!(it.next(), None);
}

#[test]
pub fn extensions_mut_insert() {
	let mut tag = langtag::LangTag::parse_copy("fr-a-ext1-b-ext2").unwrap();
	tag.extensions_mut().insert('a'.try_into().unwrap(), "ext3".try_into().unwrap());
	tag.extensions_mut().insert('c'.try_into().unwrap(), "ext4".try_into().unwrap());
	tag.extensions_mut().insert('b'.try_into().unwrap(), "ext5".try_into().unwrap());
	tag.extensions_mut().insert('c'.try_into().unwrap(), "ext6".try_into().unwrap());
	assert_eq!(tag, "fr-a-ext1-ext3-b-ext2-ext5-c-ext4-ext6")
}

#[test]
pub fn extensions_mut_remove() {
	let mut tag = langtag::LangTag::parse_copy("fr-a-ext1-ext2-b-ext3-ext4-a-ext5-a-ext6-c-ext7-a-ext8").unwrap();
	tag.extensions_mut().remove('a'.try_into().unwrap());
	assert_eq!(tag, "fr-b-ext3-ext4-c-ext7")
}

#[test]
pub fn extensions_mut_remove_subtag() {
	let mut tag = langtag::LangTag::parse_copy("fr-a-ext1-ext2-b-ext3-ext4-a-ext5-a-ext6-c-ext7-a-ext8-ext9").unwrap();
	let a: langtag::Singleton = 'a'.try_into().unwrap();
	tag.extensions_mut().remove_subtag(a, "ext1");
	assert_eq!(tag, "fr-a-ext2-b-ext3-ext4-a-ext5-a-ext6-c-ext7-a-ext8-ext9");
	tag.extensions_mut().remove_subtag(a, "ext2");
	assert_eq!(tag, "fr-b-ext3-ext4-a-ext5-a-ext6-c-ext7-a-ext8-ext9");
	tag.extensions_mut().remove_subtag(a, "ext8");
	assert_eq!(tag, "fr-b-ext3-ext4-a-ext5-a-ext6-c-ext7-a-ext9");
	tag.extensions_mut().remove_subtag(a, "ext9");
	assert_eq!(tag, "fr-b-ext3-ext4-a-ext5-a-ext6-c-ext7");
	tag.extensions_mut().remove_subtag(a, "ext5");
	assert_eq!(tag, "fr-b-ext3-ext4-a-ext6-c-ext7");
	tag.extensions_mut().remove_subtag(a, "ext6");
	assert_eq!(tag, "fr-b-ext3-ext4-c-ext7");
}