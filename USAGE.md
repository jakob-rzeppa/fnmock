# Usage

How to mock functions with fnmock, and when to use one of the two smaller attributes instead.

For the full API of a mock, see [docs/FEATURES.md](docs/FEATURES.md). For the complete
list of what is and isn't supported, see [docs/LIMITATIONS.md](docs/LIMITATIONS.md).

## What fnmock is for

Code written in a functional style, with modules of plain functions calling other plain
functions, is awkward to unit-test in isolation.

The usual fix is an object-oriented design with traits and structs. If you prefer the functional style and don't want that overhead, fnmock lets
you mock the functions themselves.

Annotate the function where it already lives, and control it directly from tests:

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
    mock.setup(|_| "Test".into());
    mock.expect(fnmock::predicate::eq(1)).once();

    assert_eq!(greet(1), "Hello, Test");

    mock.assert();
}
```

There is no trait, no dependency injection and no signature change. `greet` keeps calling
`fetch_user_name` directly. The test controls what `fetch_user_name` returns and checks how it was
called.

## Setup

The attribute is applied to production code, so fnmock is a regular dependency, not a
dev-dependency:

```toml
[dependencies]
fnmock = "<version>"
```

This costs nothing in release builds. The code the attribute injects is `#[cfg(test)]`-gated, so
outside of tests the function keeps its original body and no mock code is compiled at all.

## Test scope

The generated module and the `_mock()` accessor are `#[cfg(test)]`-gated, so they only exist in
test builds. A mock can therefore only be set up from a `#[cfg(test)]` unit test inside the crate
that defines the item. It can't be set up from an integration test under `tests/`, a doctest, or
another crate.

The accessor has the same visibility as the function it is generated for: a `pub` function has a
`pub` accessor, and a `pub(crate)` function has a `pub(crate)` accessor.

## The basics

The attribute generates an accessor named after the function, with a `_mock` suffix:

```rust
#[fnmock::mockable]
fn greet(name: String) -> String {
    format!("Real {}", name)
}

#[test]
fn test() {
    let mock = greet_mock();
    mock.expect(fnmock::predicate::eq("Test".to_string())).times(2);

    assert_eq!(greet("Test".to_string()), "Real Test");   // no closure set up -> real body

    mock.setup(|name| format!("Fake {}", name));
    assert_eq!(greet("Test".to_string()), "Fake Test");   // the closure answers ...

    mock.assert();                                        // ... and both calls were recorded
}
```

| Method | Behaviour |
| --- | --- |
| `setup(closure)` | Replace the body with a closure. Calling it again replaces the previous closure. |
| `is_set()` | Whether a closure is currently installed. |
| `expect(predicate, …)` | Expect calls matching one predicate per parameter. Returns a handle. |
| `expectf(closure)` | The same, but matching with a closure over all parameters. |
| `expect_times(n)` / `expect_once()` / `expect_never()` | Expect a total call count, regardless of arguments. |
| `assert()` | Panic unless every expectation set on this mock is fulfilled. |
| `clear()` | Reset the mock: remove the closure, and drop every expectation, `expect_times` range and recorded call. |

Each call is recorded first. Then, if a closure is installed, the closure answers instead of the
real body. Two things follow:

- A call the closure answers still counts toward expectations, whether it satisfies them or
  violates them.
- A call the closure answers never runs the real body, so the real body's side effects don't
  happen.

You don't have to use both halves. With only `setup`, `assert()` passes trivially. With only
expectations, every call runs the real body and is still recorded.

### The closure

The closure mirrors the function's signature: same parameters, same return type.

For `async` functions, the closure is an ordinary **synchronous** closure that returns the output
type. Don't return a future:

```rust
#[fnmock::mockable]
async fn fetch(id: i32) -> String { /* ... */ }

fetch_mock().setup(|id| format!("Fake {}", id));   // not `async move { ... }`
```

### Expectations

The handle returned by `expect` / `expectf` can be refined with `times`, `once`, `never`,
`describe` and `in_sequence`. A `Sequence` can require calls to happen in a particular order, even
across functions:

```rust
let seq = fnmock::Sequence::new();
get_user_mock().expect(eq("a")).once().in_sequence(&seq);
save_user_mock().expect(eq("a")).once().in_sequence(&seq);
```

A call that matches no expectation is not an error. See
[docs/FEATURES.md](docs/FEATURES.md#expectations) for everything expectations and
sequences can do.

## Methods

Applying the attribute to an inherent impl block gives every method in it its own independent mock.
The accessor becomes an associated function:

```rust
#[fnmock::mockable]
impl UserService {
    fn get(&self, id: u32) -> User { /* ... */ }
}

let mock = UserService::get_mock();
mock.setup(|_, id| User { id, name: "Test".into() });
mock.expect(fnmock::predicate::eq(1)).once();
```

The `setup` closure receives the receiver as its **first** argument, hence the leading `_` above.
Ignore it when the closure doesn't need the instance's state, or bind it when it does. This holds
for every receiver form (`&self`, `&mut self`, `self`, `Box<Self>`, `Rc<Self>`, `Pin<&mut Self>`).

Expectations never record the receiver, because it isn't a call argument the test should match on.
`expect` / `expectf` take a predicate for each remaining parameter only. Associated functions
without a receiver take no such argument in either place.

## Generics

A mock for a generic function is stored per instantiation. The accessor takes a turbofish, and each
combination of generic arguments gets its own mock, independent of the others:

```rust
#[fnmock::mockable]
fn parse<T: 'static>(input: &str) -> T { /* ... */ }

let mock = parse_mock::<u32>();   // the closure *and* the expectations apply to u32 only
mock.setup(|_| 42);
mock.expect_once();

let value = parse::<u32>("...");  // write the generic arguments out here too
```

Always write out the generic arguments on the accessor **and** at the call site. If the compiler
infers a different instantiation, the failure is **silent**. Say the mock is set up for `T = u32`
and the call resolves to `T = u64`. Nothing errors: the real body runs, the call is recorded on the
`u64` mock, and `assert()` on the `u32` mock fails on an expectation that looks like it should have
been met.

Type parameters must be `'static`, because mocks are keyed by `TypeId`. Const parameters are keyed
by value, so a mock for `foo::<5>()` leaves `foo::<7>()` unaffected. The const value isn't passed
to the closure. The closure only applies to that one value, so hardcode it.

For methods, struct generics go on the type and method generics on the accessor:

```rust
GenericService::<String>::convert_mock::<i32>().setup(|_, other| other * 2);
```

## Test isolation

Mocks live in thread-local storage, and Rust's test harness runs each `#[test]` on its own thread.
This has two consequences:

- **Tests can't leak into each other.** A mock set up in one test is invisible to every other
  test. You don't need to `clear()` a mock between tests.
- **Mocks don't cross threads at runtime.** A mock is only visible on the thread that set it up.

The second point is the one to watch. If the code under test moves work to another thread, the
mock fails there in two ways at once, both **silently**. The closure doesn't apply, so the real
body runs. The call is also recorded on that thread's own mock, not the one the test asserts on:

```rust
#[tokio::test(flavor = "multi_thread")]
async fn spawned() {
    let mock = fetch_mock();
    mock.setup(|id| format!("Fake {}", id));
    mock.expect_times(2);

    fetch(1).await;                                   // "Fake 1": same thread, recorded here
    tokio::spawn(async { fetch(1).await }).await;     // "Real 1": worker thread, recorded there

    mock.assert();   // panics: this mock only saw one call
}
```

Plain `#[tokio::test]` uses the current-thread runtime and is unaffected. Under a multi-threaded
runtime there is no guarantee that the code under test runs on the test's thread, so mocks aren't
guaranteed to work.

If a test unexpectedly hits real behaviour, or an assertion unexpectedly fails, check whether the
call crossed a thread boundary before suspecting the mock. `is_set()` helps here: assert it on the
thread that actually makes the call.

## Only need one half of a mock?

Two smaller attributes each provide one half of a mock:

- **`#[fnmock::fakeable]`** generates `<fn_name>_fake()` with `setup`, `is_set` and `clear`. It
  replaces the body and records nothing. It also accepts destructuring parameters, which a mock
  can't record. See [docs/FAKE.md](docs/FAKE.md).
- **`#[fnmock::spyable]`** generates `<fn_name>_spy()` with the expectation methods only. The real
  body always runs. It also accepts `-> impl Trait` and `-> !`, which a mock can't produce. See
  [docs/SPY.md](docs/SPY.md).
