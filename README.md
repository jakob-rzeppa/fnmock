# fnmock

A Rust mocking framework for standalone functions and methods in an impl block.

[![Crates.io](https://img.shields.io/crates/v/fnmock.svg)](https://crates.io/crates/fnmock)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

fnmock lets you mock a plain function in tests without adding a trait or dependency-injection
wiring. You annotate the function where it already lives. The test can then decide what it returns
and assert on how it was called.

```rust
#[fnmock::mockable]
fn fetch_user_name(id: u32) -> String {
    // real database call
}

fn greet(id: u32) -> String {
    format!("Hello, {}", fetch_user_name(id))
}

#[test]
fn test_greeting() {
    let mock = fetch_user_name_mock();
    mock.setup(|_| "Test".into());                  // replace what it returns ...
    mock.expect(fnmock::predicate::eq(1)).once();   // ... and expect how it is called

    assert_eq!(greet(1), "Hello, Test");

    mock.assert();
}
```

`greet` keeps calling `fetch_user_name` directly. There are no signature changes and no
indirection.

## Installation

The attribute is applied to production code, so fnmock is a regular dependency:

```toml
[dependencies]
fnmock = "<version>"
```

Everything the attribute injects is `#[cfg(test)]`-gated, so release builds keep the original
function body and compile no mock code at all. The flip side: a mock can only be set up from a
`#[cfg(test)]` unit test inside the crate that defines the annotated item. It can't be set up from
an integration test under `tests/`, a doctest, or another crate. See
[test scope](USAGE.md#test-scope) in USAGE.md.

## Overview

`#[fnmock::mockable]` works on a free function or an inherent impl block. It generates an accessor
named after the function: `<fn_name>_mock()`, or `Type::<fn_name>_mock()` for a method.

| Method | Behaviour |
| --- | --- |
| `setup(closure)` | Replace the body with a closure of the same signature. Calling it again replaces the previous one. |
| `is_set()` | Whether a replacement closure is installed. |
| `expect(predicate, …)` | Expect calls matching one predicate per parameter. |
| `expectf(closure)` | The same, but matching with a closure over all parameters. |
| `expect_times(n)` / `expect_once()` / `expect_never()` | Expect this many calls, whatever their arguments. |
| `assert()` | Panic unless every expectation is fulfilled. |
| `clear()` | Reset the mock: remove the closure, and drop every expectation and recorded call. |

Every call is recorded, including calls the `setup` closure answers. Without `setup`, the real body
runs. The handle returned by `expect` / `expectf` can be refined with `times`, `once`, `never`,
`describe` and `in_sequence`, and a `Sequence` can order calls across functions. See
[docs/FEATURES.md](docs/FEATURES.md).

Mocks are stored per thread, and the test harness gives each `#[test]` its own thread, so tests
can't leak into one another and no reset step is needed. The flip side is that a mock is only
visible on the thread that set it up. See [test isolation](USAGE.md#test-isolation) for what that
means with `tokio::spawn` and `std::thread::spawn`.

### Only need one half?

Two smaller attributes each provide one half of a mock:

- **`#[fnmock::fakeable]`** generates `<fn_name>_fake()` with `setup`, `is_set` and `clear`. It
  replaces the body and records nothing. It also accepts destructuring parameters, which a mock
  can't record. See [docs/FAKE.md](docs/FAKE.md).
- **`#[fnmock::spyable]`** generates `<fn_name>_spy()` with the expectation methods only. The real
  body always runs. It also accepts `-> impl Trait` and `-> !`, which a mock can't produce. See
  [docs/SPY.md](docs/SPY.md).

## Documentation

- **[USAGE.md](USAGE.md)**: a walkthrough of mocks covering methods and receivers, generics, and
  how test isolation works.
- **[docs/FEATURES.md](docs/FEATURES.md)**: the full reference for `#[mockable]`. It
  covers every accessor method, the fake closure, the expectation methods, sequences and their
  matching algorithm, `clear()`, and what a mock rejects.
- **[docs/FAKE.md](docs/FAKE.md)** and
  **[docs/SPY.md](docs/SPY.md)**: how `#[fakeable]` and `#[spyable]` differ
  from a mock.
- **[docs/LIMITATIONS.md](docs/LIMITATIONS.md)**: the support matrix. It lists every type, pattern,
  generic shape, impl block form and isolation rule fnmock has been tested against, with one column
  per attribute. Each cell links to the test that backs it.
- **fnmock-tests**: fnmock's test cases double as usage examples.

## License

This project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), and [LICENSE-MIT](LICENSE-MIT) for details

## Author

Jakob Rzeppa - rzeppa.jakob@gmail.com

## Repository

https://github.com/jakob-rzeppa/fnmock
