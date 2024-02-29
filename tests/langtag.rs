use langtag::NormalLangTag;

#[test]
pub fn langtag_script_region() {
	let tag = NormalLangTag::new("fr-abc-nice-bz-e-foo").unwrap();
	assert_eq!(tag, "fr-abc-nice-bz-e-foo");
	assert_eq!(tag.script().unwrap(), "nice");
	assert_eq!(tag.region().unwrap(), "bz");
}

// #[test]
// pub fn langtag_set_script() {
// 	let mut tag = NormalLangTag::new("fr-abc-bz-e-foo").unwrap();
// 	tag.set_script(Some("nice".try_into().unwrap()));
// 	assert_eq!(tag, "fr-abc-nice-bz-e-foo");
// 	assert_eq!(tag.script().unwrap(), "nice");
// 	assert_eq!(tag.region().unwrap(), "bz");
// }

// #[test]
// pub fn langtag_set_region() {
// 	let mut tag = NormalLangTag::new("fr-abc-bz-e-foo").unwrap();
// 	tag.set_region(Some("no".try_into().unwrap()));
// 	assert_eq!(tag, "fr-abc-no-e-foo");
// 	assert_eq!(tag.region().unwrap(), "no");
// 	assert_eq!(tag.extensions(), "e-foo");
// }
