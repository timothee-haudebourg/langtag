use langtag::LangTag;

#[test]
pub fn variants_eq() {
	let tag = LangTag::new("fr-azert-0foo-barbz-4242-e-ext").unwrap();
	assert_eq!(tag.variants(), "azert-0foo-barbz-4242");
}

#[test]
pub fn variants_iter() {
	let tag = LangTag::new("fr-azert-0foo-barbz-4242-e-ext").unwrap();
	let mut it = tag.variants().iter();
	assert_eq!(it.next().unwrap(), "azert");
	assert_eq!(it.next().unwrap(), "0foo");
	assert_eq!(it.next().unwrap(), "barbz");
	assert_eq!(it.next().unwrap(), "4242");
	assert_eq!(it.next(), None);
}

#[test]
pub fn variants_first() {
	let tag = LangTag::new("fr-azert-0foo-barbz-4242-e-ext").unwrap();
	assert_eq!(tag.variants().first().unwrap(), "azert");
}

#[test]
pub fn variants_last() {
	let tag = LangTag::new("fr-azert-0foo-barbz-4242-e-ext").unwrap();
	assert_eq!(tag.variants().last().unwrap(), "4242");
}

// #[test]
// pub fn variants_mut_first() {
// 	let mut tag = langtag::LangTag::parse_copy("fr-azert-0foo-barbz-4242-e-ext").unwrap();
// 	assert_eq!(tag.variants_mut().first().unwrap(), "azert");
// }

// #[test]
// pub fn variants_mut_last() {
// 	let mut tag = langtag::LangTag::parse_copy("fr-azert-0foo-barbz-4242-e-ext").unwrap();
// 	assert_eq!(tag.variants_mut().last().unwrap(), "4242");
// }

// #[test]
// pub fn variants_push() {
// 	let mut tag = langtag::LangTag::parse_copy("fr-azert-0foo-barbz-e-ext").unwrap();
// 	tag.variants_mut().push("4242".try_into().unwrap());
// 	let mut it = tag.variants().iter();
// 	assert_eq!(it.next().unwrap(), "azert");
// 	assert_eq!(it.next().unwrap(), "0foo");
// 	assert_eq!(it.next().unwrap(), "barbz");
// 	assert_eq!(it.next().unwrap(), "4242");
// 	assert_eq!(it.next(), None);
// }

// #[test]
// pub fn variants_pop() {
// 	let mut tag = langtag::LangTag::parse_copy("fr-azert-0foo-barbz-4242-e-ext").unwrap();
// 	assert_eq!(tag.variants_mut().last().unwrap(), "4242");
// 	tag.variants_mut().pop();
// 	assert_eq!(tag.variants_mut().last().unwrap(), "barbz");
// 	let mut it = tag.variants().iter();
// 	assert_eq!(it.next().unwrap(), "azert");
// 	assert_eq!(it.next().unwrap(), "0foo");
// 	assert_eq!(it.next().unwrap(), "barbz");
// 	assert_eq!(it.next(), None);

// 	tag.variants_mut().pop();
// 	assert_eq!(tag.variants_mut().last().unwrap(), "0foo");
// 	let mut it = tag.variants().iter();
// 	assert_eq!(it.next().unwrap(), "azert");
// 	assert_eq!(it.next().unwrap(), "0foo");
// 	assert_eq!(it.next(), None);

// 	tag.variants_mut().pop();
// 	let mut it = tag.variants().iter();
// 	assert_eq!(it.next().unwrap(), "azert");
// 	assert_eq!(it.next(), None);

// 	tag.variants_mut().pop();
// 	assert_eq!(tag.variants_mut().last(), None);
// 	let mut it = tag.variants().iter();
// 	assert_eq!(it.next(), None);
// }
