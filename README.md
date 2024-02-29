# Language Tags

[![CI](https://github.com/timothee-haudebourg/langtag/workflows/CI/badge.svg)](https://github.com/timothee-haudebourg/langtag/actions)
[![Crate informations](https://img.shields.io/crates/v/langtag.svg?style=flat-square)](https://crates.io/crates/langtag)
[![License](https://img.shields.io/crates/l/langtag.svg?style=flat-square)](https://github.com/timothee-haudebourg/langtag#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/langtag)

<!-- cargo-rdme start -->

This crate provides an implementation of *language tags* defined by
[RFC5646] ([BCP47]).

[RFC5646]: <https://tools.ietf.org/html/rfc5646>
[BCP47]: <https://tools.ietf.org/html/bcp47>

### Usage

You can easily parse new language from any string:
```rust
use langtag::LangTag;

fn main() -> Result<(), langtag::InvalidLangTag<&'static str>> {
  let tag = LangTag::new("fr-FR")?;
  assert_eq!(tag.language().unwrap().primary(), "fr");
  assert!(tag == "Fr-fr"); // comparison is case-insensitive.
  Ok(())
}
```

Note that `LangTag::new` does *not* copy the data it is given,
but only borrows it. The `LangTagBuf` type allows you to own the language
tag. Once parsed, you can explore every component of the language tag using
the provided functions.


<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
