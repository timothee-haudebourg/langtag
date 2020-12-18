# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unpublished]
### Changed
- `LanguageTagBuf::as_ref` now returns a `LanguageTag<&[u8]>`.
- Add `LanguageTag::cloned`.
- Impl `Copy` for `LanguageTag`, `LanguageTagBuf` and `PrivateUseTag`.

## [0.1.1]
### Changed
- Impl `Clone` for `LanguageTag`, `LanguageTagBuf` and `PrivateUseTag`.
- Impl `AsRef<[u8]>`, `AsRef<str>`, `PartialEq<U>`, `Eq`, `fmt::Display` and `fmt::Debug` for `LanguageTagBuf`.
- Impl `Hash`, `PartialOrd` and `Ord` for `LanguageTag` and `LanguageTagBuf`.