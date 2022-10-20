# Language Tags

[![CI](https://github.com/timothee-haudebourg/langtag/workflows/CI/badge.svg)](https://github.com/timothee-haudebourg/langtag/actions)
[![Crate informations](https://img.shields.io/crates/v/langtag.svg?style=flat-square)](https://crates.io/crates/langtag)
[![License](https://img.shields.io/crates/l/langtag.svg?style=flat-square)](https://github.com/timothee-haudebourg/langtag#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/langtag)

This crate provides an implementation of *language tags* defined by
[RFC5646](https://tools.ietf.org/html/rfc5646) ([BCP47](https://tools.ietf.org/html/bcp47)).

### Usage

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

Note that [`LanguageTag::parse`] does *not* copy the data it is given,
but only borrows it.
You can create an owning `LanguageTag` instance by using
[`LanguageTagBuf::parse_copy`] to copy the data,
or simply [`LanguageTagBuf::new`] to move the data.

Once parsed, you can explore every component of the language tag using the provided functions.

#### Mutable language tags

When the language tags owns its buffer through `Vec<u8>`,
it becomes possible to access the tag mutably to modify it.
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
