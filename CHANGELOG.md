# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The `fnmock` and `fnmock-derive` crates share a single version and are released
together.

## [Unreleased]

- Fakes match the visibility of the faked function, instead of being always `pub(crate)`.
- Fake interface methods `setup` and `clear` use `&self` instead of `self`, so they can't be chained anymore and `let fake = fake.clear()` doesn't work anymore.
- Fake interface method `get` is removed, as it was a internal implementation detail and not part of the public API.

## [0.1.0] - 2026-07-22

### Added

- Initial release of `fnmock`, a mocking framework for standalone functions and
  methods in an `impl` block.
- Initial release of `fnmock-derive` - a procedural macro crate powering `fnmock`.

[Unreleased]: https://github.com/jakob-rzeppa/fnmock/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jakob-rzeppa/fnmock/releases/tag/v0.1.0
