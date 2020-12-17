use langtag::LanguageTag;

#[test]
pub fn variants_eq() {
	let tag = LanguageTag::parse("fr-azert-0foo-barbz-4242-e-ext").unwrap();
	assert_eq!(tag.variants(), "azert-0foo-barbz-4242");
}

#[test]
pub fn variants_iter() {
	let tag = LanguageTag::parse("fr-azert-0foo-barbz-4242-e-ext").unwrap();
	let mut it = tag.variants().iter();
	assert_eq!(it.next().unwrap(), "azert");
	assert_eq!(it.next().unwrap(), "0foo");
	assert_eq!(it.next().unwrap(), "barbz");
	assert_eq!(it.next().unwrap(), "4242");
	assert_eq!(it.next(), None);
}

#[test]
pub fn variants_first() {
	let tag = LanguageTag::parse("fr-azert-0foo-barbz-4242-e-ext").unwrap();
	assert_eq!(tag.variants().first().unwrap(), "azert");
}

#[test]
pub fn variants_last() {
	let tag = LanguageTag::parse("fr-azert-0foo-barbz-4242-e-ext").unwrap();
	assert_eq!(tag.variants().last().unwrap(), "4242");
}