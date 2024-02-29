use langtag::{LangTag, NormalLangTag};

#[test]
pub fn language_eq() {
	let tag = LangTag::new("fr-abc-def-ghi-bz").unwrap();
	assert_eq!(tag.language().unwrap(), "fr-abc-def-ghi");
}

#[test]
pub fn language_primary_eq() {
	let tag = NormalLangTag::new("fr-abc-def-ghi-bz").unwrap();
	assert_eq!(tag.language().primary(), "fr");
}

// #[test]
// pub fn language_primary_set() {
// 	let mut tag = NormalLangTag::new("fr-abc-def-ghi-bz").unwrap();
// 	tag.language_mut().set_primary("foo".try_into().unwrap());
// 	assert_eq!(tag, "foo-abc-def-ghi-bz");
// 	assert_eq!(tag.language().extension().unwrap(), "abc-def-ghi");
// 	assert_eq!(tag.region().unwrap(), "bz");
// }

// #[test]
// pub fn language_primary_set_long() {
// 	let mut tag = langtag::LangTag::parse_copy("fr-abc-def-ghi-bz").unwrap();
// 	tag.language_mut().set_primary("foobar".try_into().unwrap());
// 	assert_eq!(tag, "foobar-bz");
// 	assert_eq!(tag.language().extension(), None);
// 	assert_eq!(tag.region().unwrap(), "bz");
// }

#[test]
pub fn language_ext_eq() {
	let tag = NormalLangTag::new("fr-abc-def-ghi-bz").unwrap();
	assert_eq!(tag.language().extension().unwrap(), "abc-def-ghi");
}

// #[test]
// pub fn language_ext_mut_insert() {
// 	let mut tag = langtag::LangTag::parse_copy("fr-abc-bz").unwrap();
// 	assert_eq!(
// 		tag.language_mut()
// 			.extension_mut()
// 			.unwrap()
// 			.insert("def".try_into().unwrap()),
// 		true
// 	);
// 	assert_eq!(tag, "fr-abc-def-bz");
// 	assert_eq!(tag.region().unwrap(), "bz");
// }

// #[test]
// pub fn language_ext_mut_remove() {
// 	let mut tag = langtag::LangTag::parse_copy("fr-abc-def-ghi-bz").unwrap();
// 	assert_eq!(
// 		tag.language_mut().extension_mut().unwrap().remove("def"),
// 		true
// 	);
// 	assert_eq!(tag, "fr-abc-ghi-bz");
// 	assert_eq!(tag.region().unwrap(), "bz");
// }
