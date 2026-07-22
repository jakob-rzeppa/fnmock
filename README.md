# fnmock

A Rust mocking framework for standalone functions and methods in an impl block.

[![Crates.io](https://img.shields.io/crates/v/fnmock.svg)](https://crates.io/crates/fnmock)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

fnmock lets you replace a function's behaviour in tests without introducing a trait / dependency injection wiring. You annotate the function where it already
lives, and the test controls what it returns.

```rust
#[fnmock::fakeable]
fn fetch_user_name(id: u32) -> String {
    todo!()
}

fn greet(id: u32) -> String {
    format!("Hello, {}", fetch_user_name(id))
}

#[test]
fn test_greeting() {
    fetch_user_name_fake().setup(|_| "Test".into());

    assert_eq!(greet(1), "Hello, Test");
}
```

`greet` keeps calling `fetch_user_name` directly — no signature changes, no indirection.

## Installation

The attribute is applied to production code, so fnmock is a regular dependency:

```toml
[dependencies]
fnmock = "<version>"
```

The fake lookup is `#[cfg(test)]`-gated, so release builds keep the original function body and
compile no fake machinery at all. The flip side: fakes can only be installed from a `#[cfg(test)]`
unit test inside the crate that defines the fakeable item — not from an integration test under
`tests/`, a doctest, or another crate. See [test scope](USAGE.md#test-scope) in USAGE.md.

## Documentation

- **[USAGE.md](USAGE.md)** — how to use fakes: the accessor API, methods and receivers, generics,
  and how test isolation works.
- **[FEATURES.md](FEATURES.md)** — the user-facing API: the `#[fnmock::fakeable]` attribute, the
  `_fake()` accessor and its call shapes, and the `setup`/`clear`/`is_set` methods.
- **[CONSTRAINTS.md](CONSTRAINTS.md)** — the full supported surface (types, patterns, `async`/
  `unsafe`/`extern`, generics, impl blocks), the isolation/keying rules, and the cases that are
  explicitly rejected at compile time.
- **fnmock-tests** - fnmock's test cases can be useful for examples on how to use this project.

## Overview

Applying `#[fnmock::fakeable]` to a function or an inherent impl block generates an accessor named
after it:

| Method | Behaviour |
| --- | --- |
| `setup(closure)` | Install a fake, replacing any previous one. |
| `clear()` | Remove the fake; calls run the real body again. |
| `is_set()` | Whether a fake is currently installed. |

In an impl block every method is faked and accessors are generated as associated functions.

Fakes are stored per thread, and the test harness gives each `#[test]` its own thread, so tests
cannot leak into one another and no reset step is needed. The flip side is that a fake is only
visible on the thread that installed it — see
[test isolation](USAGE.md#test-isolation) for what that means around `tokio::spawn` and
`std::thread::spawn`.

## Work in Progress

- Spies
- Mocks

## License

This project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), and [LICENSE-MIT](LICENSE-MIT) for details

## Author

Jakob Rzeppa - rzeppa.jakob@gmail.com

## Repository

https://github.com/jakob-rzeppa/fnmock
