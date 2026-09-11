# fnmock

A Rust mocking framework for standalone functions and methods in an impl block.

[![Crates.io](https://img.shields.io/crates/v/fnmock.svg)](https://crates.io/crates/fnmock)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

fnmock lets you replace or observe a function's behaviour in tests without introducing a trait /
dependency injection wiring. You annotate the function where it already lives, and the test
controls what it returns or asserts on how it was called.

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

A **spy** works the other way round: the real body still runs, but the test can assert on which
arguments it was called with.

```rust
#[fnmock::spyable]
fn fetch_user_name(id: u32) -> String {
    format!("user {id}")
}

#[test]
fn test_fetch_user_name() {
    let spy = fetch_user_name_spy();
    spy.expect(fnmock::predicate::eq(1)).once();

    assert_eq!(fetch_user_name(1), "user 1"); // the real body still runs

    spy.assert();
}
```

## Installation

The attribute is applied to production code, so fnmock is a regular dependency:

```toml
[dependencies]
fnmock = "<version>"
```

The fake lookup and the spy's call recording are both `#[cfg(test)]`-gated, so release builds keep
the original function body and compile no fake or spy machinery at all. The flip side: fakes and
spies can only be set up from a `#[cfg(test)]` unit test inside the crate that defines the
annotated item — not from an integration test under `tests/`, a doctest, or another crate. See
[test scope](USAGE.md#test-scope) in USAGE.md.

## Documentation

- **[USAGE.md](USAGE.md)** — how to use fakes: the accessor API, methods and receivers, generics,
  and how test isolation works.
- **[docs/FAKE_FEATURES.md](docs/FAKE_FEATURES.md)** — what is specific to `#[fnmock::fakeable]`:
  the attribute, the `_fake()` accessor, and `setup`/`clear`/`is_set`.
- **[docs/SPY_FEATURES.md](docs/SPY_FEATURES.md)** — what is specific to `#[fnmock::spyable]`: the
  attribute, the `_spy()` accessor, the expectation methods, and the expectation DSL: `times`,
  global counts, sequences and the matching algorithm.
- **[docs/LIMITATIONS.md](docs/LIMITATIONS.md)** — the shared support matrix: every type, pattern,
  generic shape, impl block form and isolation rule fnmock has been tested against, with a column
  for fakes and a column for spies, each cell linked to the test that backs it — so both what works
  and where the two macros differ are visible at a glance.
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

Applying `#[fnmock::spyable]` instead generates a `<fn_name>_spy()` accessor (or, in an impl block,
`Type::<fn_name>_spy()`) that lets the test set expectations on the arguments a call is made with,
without changing what the function does:

| Method | Behaviour |
| --- | --- |
| `expect(predicate, …)` | Expect calls matching one predicate per parameter. |
| `expectf(closure)` | Same, but matching with a closure over all parameters. |
| `expect_times(n)` / `expect_once()` / `expect_never()` | Expect this many calls, whatever their arguments. |
| `assert()` | Panic unless every expectation set on this spy is fulfilled. |

The handle returned by `expect`/`expectf` can be refined further with `times`, `once`, `never`,
`describe` and `in_sequence` — see [docs/SPY_FEATURES.md](docs/SPY_FEATURES.md) for the full
expectation DSL.

Both fakes and spies are stored per thread, and the test harness gives each `#[test]` its own
thread, so tests cannot leak into one another and no reset step is needed. The flip side is that a
fake or spy is only visible on the thread that installed it — see
[test isolation](USAGE.md#test-isolation) for what that means around `tokio::spawn` and
`std::thread::spawn`.

## Work in Progress

- Mocks

## License

This project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), and [LICENSE-MIT](LICENSE-MIT) for details

## Author

Jakob Rzeppa - rzeppa.jakob@gmail.com

## Repository

https://github.com/jakob-rzeppa/fnmock
