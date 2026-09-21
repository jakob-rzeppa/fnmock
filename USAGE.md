# Usage

How to use fnmock's fakes and spies, and when reaching for each is the right call.

For the fake-specific API — the attribute, the accessor and `setup`/`clear`/`is_set` — see
[docs/FAKE_FEATURES.md](docs/FAKE_FEATURES.md), and [docs/SPY_FEATURES.md](docs/SPY_FEATURES.md) for
the spy-specific equivalent. For the exhaustive list of what is and isn't supported by each, see
[docs/LIMITATIONS.md](docs/LIMITATIONS.md).

## What fnmock is for

Rust code written in a functional style — modules of plain functions calling other plain
functions — is awkward to unit-test in isolation.

The conventional way to fix this is using object-oriented design with traits and structs.
These can be mocked via `mockall` or similar crates. But if you like a functional
programming style and don't want to add all this overhead, `fnmock` can give you the
possibility to use the functions by themselves.

Just annotate the function where it already lives and replace it directly in tests:

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

No trait, no dependency injection, no signature change. `greet` keeps calling `fetch_user_name`
directly, and the test controls what it returns.

The `#[fnmock::spyable]` counterpart: the real body still runs, and the test
asserts on what it was called with instead of on what it returns:

```rust
#[fnmock::spyable]
fn fetch_user_name(id: u32) -> String {
    format!("user {id}")
}

#[test]
fn test_fetch_user_name() {
    let spy = fetch_user_name_spy();
    spy.expect(fnmock::predicate::eq(1)).once();

    assert_eq!(fetch_user_name(1), "user 1");   // the real body still runs

    spy.assert();
}
```

## Setup

Both `#[fnmock::fakeable]` and `#[fnmock::spyable]` are applied to production code, so fnmock is a
regular dependency, not a dev-dependency:

```toml
[dependencies]
fnmock = "<version>"
```

This costs nothing in release builds. The fake lookup and the spy's call recording that the macros
inject are both `#[cfg(test)]`-gated, so outside of tests the function keeps its original body and
no fake or spy machinery is compiled at all.

## Test scope

The fake module and `_fake()` accessor, and the spy module and `_spy()` accessor, are all
`#[cfg(test)]`-gated, so they are only available in test builds. They match the visibility of the
function they are generated for, so a `pub` function has a `pub` accessor and a `pub(crate)`
function has a `pub(crate)` accessor.

## Fakes: the basics

The attribute generates an accessor named after the function, suffixed with `_fake`:

```rust
#[fnmock::fakeable]
fn greet(name: String) -> String {
    format!("Real {}", name)
}
#[test]
fn test() {
    assert_eq!(greet("Test".to_string()), "Real Test");   // no fake set -> real body

    greet_fake().setup(|name| format!("Fake {}", name));
    assert_eq!(greet("Test".to_string()), "Fake Test");   // fake intercepts

    greet_fake().clear();
    assert_eq!(greet("Test".to_string()), "Real Test");   // back to the real body
}
```

| Method | Behaviour |
| --- | --- |
| `setup(closure)` | Install a fake. Calling it again replaces the previous one. |
| `clear()` | Remove the fake. |
| `is_set()` | Whether a fake is currently installed. |

The closure mirrors the function's signature — same parameters, same return type. Parameter
patterns carry over too, so a function taking `(left, right): (String, String)` is faked with
`setup(|(left, right)| ...)`.

For `async` functions, the closure is an ordinary **synchronous** closure returning the output
type. Don't return a future:

```rust
#[fnmock::fakeable]
async fn fetch(id: i32) -> String { /* ... */ }

fetch_fake().setup(|id| format!("Fake {}", id));   // not `async move { ... }`
```

## Spies: the basics

The attribute generates an accessor named after the function, suffixed with `_spy`:

```rust
#[fnmock::spyable]
fn greet(name: String) -> String {
    format!("Real {}", name)
}

#[test]
fn test() {
    let spy = greet_spy();
    spy.expect(fnmock::predicate::eq("Test".to_string())).once();

    assert_eq!(greet("Test".to_string()), "Real Test");   // the real body always runs

    spy.assert();
}
```

| Method | Behaviour |
| --- | --- |
| `expect(predicate, …)` | Expect calls matching one predicate per parameter. Returns a handle. |
| `expectf(closure)` | Same, but matching with a closure over all parameters. |
| `expect_times(n)` / `expect_once()` / `expect_never()` | Expect a total call count, regardless of arguments. |
| `assert()` | Panic unless every expectation set on this spy is fulfilled. |

There is no `clear()` or `is_set()`; a spy has no installed state to remove — it never changes
what the function does. The handle returned by `expect`/`expectf` can be refined with `times`,
`once`, `never`, `describe` and `in_sequence`; see
[docs/SPY_FEATURES.md](docs/SPY_FEATURES.md) for the full expectation DSL, including how
sequences order calls across functions.

## Methods

Applying either attribute to an inherent impl block makes every method in it fakeable or spyable,
each with its own independent fake or spy. The accessor becomes an associated function:

```rust
#[fnmock::fakeable]
impl UserService {
    fn get(&self, id: u32) -> User { /* ... */ }
}

UserService::get_fake().setup(|_, id| User { id, name: "Test".into() });
```

```rust
#[fnmock::spyable]
impl UserService {
    fn get(&self, id: u32) -> User { /* ... */ }
}

UserService::get_spy().expect(fnmock::predicate::eq(1)).once();
```

For a fake, the receiver is passed as the **first** closure argument — hence the leading `_`
above. Ignore it when the fake doesn't care about the instance state, or bind it when it does.
This holds for every receiver form (`&self`, `&mut self`, `self`, `Box<Self>`, `Rc<Self>`,
`Pin<&mut Self>`). A spy never records the receiver — it isn't a call argument the test should match
on — so `expect`/`expectf` only take a predicate per remaining parameter.

Associated functions without a receiver take no such argument, on either side.

## Generics

Fakes and spies for generic functions are both stored per instantiation, so the accessor takes a
turbofish and each combination of generic arguments gets its own fake or spy, independent of the
others:

```rust
#[fnmock::fakeable]
fn parse<T: 'static>(input: &str) -> T { /* ... */ }

parse_fake::<u32>().setup(|_| 42);
```

```rust
#[fnmock::spyable]
fn parse<T: 'static>(input: &str) -> T { /* ... */ }

parse_spy::<u32>().expect_once();
```

Since you need to specify a fake or a set of expectations for each combination of generics, make
sure to always specify the generics when using the accessor. The compiler might infer the wrong
types and you are left debugging.

It is also recommended to specify the generics on calls of the faked or spied function, to be sure
the fake's/spy's generics match the used ones. For simple functions this might be unnecessary, but
with complexity it is more likely the implementation will not apply to the used generics.

For a fake this matters because the failure is **silent**. If the fake is registered for
`T = u32` and the call site resolves to `T = u64`, nothing errors — the real implementation just
runs and your test quietly exercises production code:

```rust
parse_fake::<u32>().setup(|_| 42);
let value = parse::<u32>("...");   // be explicit here too
```

For a spy the failure is just as silent, but the other way round: a call resolving to a different
instantiation than the one you set expectations on is never recorded, so `spy.assert()` fails on
an expectation that looks like it should have been satisfied.

Type parameters must be `'static`, since fakes and spies are keyed by `TypeId`. Const parameters
are keyed by value, so a fake or spy for `foo::<5>()` leaves `foo::<7>()` unaffected. The const
value isn't accessible inside a fake's closure — hardcode it, as the fake only applies to that one
value.

For methods, struct generics go on the type and method generics on the accessor:

```rust
GenericService::<String>::convert_fake::<i32>().setup(|_, other| other * 2);
GenericService::<String>::convert_spy::<i32>().expect_once();
```

## Test isolation

Fakes and spies both live in thread-local storage, and Rust's test harness runs each `#[test]` on
its own thread. Two consequences, for either:

- **Tests can't leak into each other.** A fake or spy set up in one test is invisible to every
  other test. You do not need to `clear()` a fake between tests.
- **Fakes and spies don't cross threads at runtime.** A fake or spy is only visible on the thread
  that set it up.

The second point is the one to watch. If the code under test moves work to another thread, a fake
won't apply there — and because an unset fake falls through to the real implementation, this fails
*silently* rather than erroring:

```rust
#[tokio::test(flavor = "multi_thread")]
async fn spawned() {
    fetch_fake().setup(|id| format!("Fake {}", id));

    fetch(1).await;                                   // "Fake 1"  — same thread
    tokio::spawn(async { fetch(1).await }).await;     // "Real 1"  — worker thread
}
```

A spy fails just as silently, but the other way round: a call made on another thread is never
recorded on the spy set up on the test's thread, so `spy.assert()` fails on an expectation that
looks like it should have seen the call:

```rust
#[tokio::test(flavor = "multi_thread")]
async fn spawned() {
    let spy = fetch_spy();
    spy.expect_once();

    tokio::spawn(async { fetch(1).await }).await;     // recorded on the worker thread's spy, not this one

    spy.assert();   // panics: this spy never saw a call
}
```

Plain `#[tokio::test]` uses the current-thread runtime and is unaffected. Under a multi-threaded
runtime, it can't be guaranteed that the code under test runs on the same thread as the test itself, so fakes and spies are not guaranteed to work.

If a test unexpectedly hits real behaviour or a spy assertion fails unexpectedly, check whether
the call crossed a thread boundary before suspecting the fake or spy. `is_set()` is useful here
for fakes: assert it on the thread that actually makes the call.
