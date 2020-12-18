# Language Tags

<table><tr>
	<td><a href="https://docs.rs/langtag">Documentation</a></td>
	<td><a href="https://crates.io/crates/langtag">Crate informations</a></td>
	<td><a href="https://github.com/timothee-haudebourg/langtag">Repository</a></td>
</tr></table>

This crate provides an implementation of *language tags* defined by
[RFC5646](https://tools.ietf.org/html/rfc5646) ([BCP47](https://tools.ietf.org/html/bcp47)).

## Usage

You can easily parse new language from anything that provides a `[u8]` reference:
```rust
extern crate langtag;
use langtag::LanguageTag;

fn main() -> Result<(), langtag::Error> {
	let tag = LanguageTag::parse("fr-FR")?;
	assert_eq!(tag.language().unwrap().primary(), "fr");
	assert!(tag == "Fr-fr"); // comparison is case-insensitive.
	Ok(())
}
```

Note that [`LanguageTag::parse`][1] does *not* copy the data it is given,
but only borrows it.
You can create an owning `LanguageTag` instance by using
[`LanguageTagBuf::parse_copy`][2] instead.

Once parsed, you can exlore every component of the language tag using the provided functions.

[1] https://docs.rs/langtag/latest/langtag/enum.LanguageTag.html#method.parse
[2] https://docs.rs/langtag/latest/langtag/enum.LanguageTagBuf.html#method.parse_copy

### Mutable language tags

When the language tags owns its buffer through `Vec<u8>`,
it becomes possible to access the tag mutabily to modify it.
```rust
extern crate langtag;
use std::convert::TryInto;
use langtag::LangTag;

fn main() -> Result<(), langtag::Error> {
	let mut tag = LangTag::parse_copy("fr-FR")?;
	tag.language_mut().set_primary("jp".try_into()?);
	tag.set_region(None);
	tag.extensions_mut().insert('f'.try_into()?, "bar".try_into()?);
	assert_eq!(tag, "jp-f-bar");
	Ok(())
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
