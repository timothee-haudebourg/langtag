use langtag::LangTag;

#[test]
pub fn privateuse_eq() {
	let tag = LangTag::new("fr-x-ext1-ext2-ext3-ext4-ext5-ext6").unwrap();
	assert_eq!(
		tag.private_use().map(AsRef::as_ref),
		Some("x-ext1-ext2-ext3-ext4-ext5-ext6")
	);
}

#[test]
pub fn privateuse_iter() {
	let tag = LangTag::new("fr-x-ext1-ext2-ext3-ext4-ext5-ext6").unwrap();
	let mut it = tag.private_use_subtags();
	assert_eq!(it.next().unwrap(), "ext1");
	assert_eq!(it.next().unwrap(), "ext2");
	assert_eq!(it.next().unwrap(), "ext3");
	assert_eq!(it.next().unwrap(), "ext4");
	assert_eq!(it.next().unwrap(), "ext5");
	assert_eq!(it.next().unwrap(), "ext6");
	assert_eq!(it.next(), None);
}

// #[test]
// pub fn privateuse_insert1() {
// 	let mut tag = NormalLangTag::new("fr").unwrap();
// 	let mut pu = tag.private_use_subtags_mut();
// 	pu.insert("ext1".try_into().unwrap());
// 	assert_eq!(tag, "fr-x-ext1");
// }

// #[test]
// pub fn privateuse_insert2() {
// 	let mut tag = NormalLangTag::new("fr").unwrap();
// 	let mut pu = tag.private_use_subtags_mut();
// 	pu.insert("ext1".try_into().unwrap());
// 	pu.insert("ext2".try_into().unwrap());
// 	assert_eq!(tag, "fr-x-ext1-ext2");
// }

// #[test]
// pub fn privateuse_insert3() {
// 	let mut tag = NormalLangTag::new("fr-x-ext1-ext2-ext3").unwrap();
// 	let mut pu = tag.private_use_subtags_mut();
// 	assert_eq!(pu.insert("ext1".try_into().unwrap()), false);
// 	assert_eq!(pu.insert("ext2".try_into().unwrap()), false);
// 	assert_eq!(pu.insert("ext3".try_into().unwrap()), false);
// 	assert_eq!(pu.insert("ext4".try_into().unwrap()), true);
// 	assert_eq!(tag, "fr-x-ext1-ext2-ext3-ext4");
// }

// #[test]
// pub fn privateuse_remove1() {
// 	let mut tag = NormalLangTag::new("fr-x-ext1-ext2-ext2").unwrap();
// 	let mut pu = tag.private_use_subtags_mut();
// 	pu.remove("ext2");
// 	assert_eq!(tag, "fr-x-ext1");
// }

// #[test]
// pub fn privateuse_remove2() {
// 	let mut tag = NormalLangTag::new("fr-x-ext1-ext2-ext2").unwrap();
// 	let mut pu = tag.private_use_subtags_mut();
// 	pu.remove("ext2");
// 	pu.remove("ext1");
// 	assert_eq!(tag, "fr");
// }
