# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

The `fnmock` and `fnmock-derive` crates share a single version and are released
together.

## [Unreleased]

## [0.2.1] - 2026-09-23

### Added

- `clear` method on spies to reset the call history, expectations and sequences.

## [0.2.0] - 2026-09-22

### Added

- `fnmock` now supports spying on functions and methods by the `spyable` attribute.

### Changed

- Fakes match the visibility of the faked function, instead of being always `pub(crate)`.
- Fake interface methods `setup` and `clear` use `&self` instead of `self`, so they can't be chained anymore and `let fake = fake.clear()` doesn't work anymore.

### Removed

- Fake interface method `get` is removed, as it was a internal implementation detail and not part of the public API.

### Fixed

- `a::Struct` and `b::Struct` impls in the same module no longer collide, even if they have the same name.
- `Struct<u8>` and `Struct<u16>` impls in the same module no longer collide, even if they have the same function names.
- A type parameter's `'static` bound is now recognised when it is reached through a named lifetime (`T: 'a` with `'a: 'static`), including through a chain of lifetimes and when either half is written in the `where` clause.
- A `where` predicate's `for<..>` binder is no longer dropped when the predicate is merged into its parameter, so `where for<'x> F: Fn(&'x str) -> String` compiles.
- A bound that names a lifetime which will not be in scope on the generated items is now rejected with a spanned error instead of expanding to a bare `use of undeclared lifetime name`.

## [0.1.0] - 2026-07-22

### Added

- Initial release of `fnmock`, a mocking framework for standalone functions and
  methods in an `impl` block.
- Initial release of `fnmock-derive` - a procedural macro crate powering `fnmock`.

[Unreleased]: https://github.com/jakob-rzeppa/fnmock/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/jakob-rzeppa/fnmock/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/jakob-rzeppa/fnmock/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jakob-rzeppa/fnmock/releases/tag/v0.1.0
