# Usage

How to use fnmock's fakes, and when reaching for them is the right call.

For the exhaustive list of what is and isn't supported, see [FEATURES.md](FEATURES.md).

## What fnmock is for

Rust code written in a functional style — modules of plain functions calling other plain
functions — is awkward to unit-test in isolation. The conventional fix is to introduce a seam:
define a trait, implement it, pass it around as `Box<dyn Trait>` or a generic parameter, and wire
dependencies in through constructors.

That seam exists purely so a mocking library can substitute behaviour. It changes your function
signatures, adds indirection, and pushes an object-oriented shape onto the code.

I do not want to start a debate wether one or the other code style is better. This should be kept to the person
maintaining the code. This project only exists to give you the opportunity to choose what is right for you.

If you decide to use fnmock: annotate the function where it already lives and replace it directly in tests:

```rust
#[fnmock::fakeable]
fn fetch_user(id: u32) -> User {
    // real database call
}

fn greet(id: u32) -> String {
    format!("Hello, {}", fetch_user(id).name)
}

#[test]
fn test_greeting() {
    fetch_user_fake().setup(|id| User { id, name: "Test".into() });

    assert_eq!(greet(1), "Hello, Test");
}
```

No trait, no dependency injection, no signature change. `greet` keeps calling `fetch_user`
directly, and the test controls what it returns.

## Setup

`#[fnmock::fakeable]` is applied to production code, so fnmock is a regular dependency, not a
dev-dependency:

```toml
[dependencies]
fnmock = "0.1.0"
```

This costs nothing in release builds. The fake lookup the macro injects is `#[cfg(test)]`-gated,
so outside of tests the function keeps its original body and no fake machinery is compiled at all.

## The basics

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

## Methods

Applying the attribute to an inherent impl block makes every method in it fakeable, each with its
own independent fake. The accessor becomes an associated function:

```rust
#[fnmock::fakeable]
impl UserService {
    fn get(&self, id: u32) -> User { /* ... */ }
}

UserService::get_fake().setup(|_, id| User { id, name: "Test".into() });
```

The receiver is passed as the **first** closure argument — hence the leading `_` above. Ignore it
when the fake doesn't care about the instance state, or bind it when it does. This holds for every
receiver form (`&self`, `&mut self`, `self`, `Box<Self>`, `Rc<Self>`, `Pin<&mut Self>`).

Associated functions without a receiver take no such argument.

## Generics

Fakes for generic functions are stored per instantiation, so the accessor takes a turbofish and
each combination of generic arguments is faked independently:

```rust
#[fnmock::fakeable]
fn parse<T: 'static>(input: &str) -> T { /* ... */ }

parse_fake::<u32>().setup(|_| 42);
```

Since you need to specify fakes for each combination of generics, make sure to always specify the
generics when using the fake. The compiler might infer the wrong types and you are left debugging.

It is also recommended to specify the generics on calls of the faked function, to be sure the
fake's generics match the used ones. For simple functions this might be unnecessary, but with
complexity it is more likely the fake implementation will not fake for the used generics.

This matters because the failure is **silent**. If the fake is registered for `T = u32` and the
call site resolves to `T = u64`, nothing errors — the real implementation just runs and your test
quietly exercises production code:

```rust
parse_fake::<u32>().setup(|_| 42);
let value = parse::<u32>("...");   // be explicit here too
```

Type parameters must be `'static`, since fakes are keyed by `TypeId`. Const parameters are keyed
by value, so a fake for `foo::<5>()` leaves `foo::<7>()` running the real body. The const value
isn't accessible inside the closure — hardcode it, as the fake only applies to that one value.

For methods, struct generics go on the type and method generics on the accessor:

```rust
GenericService::<String>::convert_fake::<i32>().setup(|_, other| other * 2);
```

## Test isolation

Fakes live in thread-local storage, and Rust's test harness runs each `#[test]` on its own thread.
Two consequences:

- **Tests can't leak into each other.** A fake set in one test is invisible to every other test,
  including under `--test-threads=1`, where the harness still allocates a fresh thread per test.
  You do not need to `clear()` between tests, and you don't need `#[serial]`.
- **Fakes don't cross threads at runtime.** A fake is only visible on the thread that set it up.

The second point is the one to watch. If the code under test moves work to another thread, the
fake won't apply there — and because an unset fake falls through to the real implementation, this
fails *silently* rather than erroring:

```rust
#[tokio::test(flavor = "multi_thread")]
async fn spawned() {
    fetch_fake().setup(|id| format!("Fake {}", id));

    fetch(1).await;                                   // "Fake 1"  — same thread
    tokio::spawn(async { fetch(1).await }).await;     // "Real 1"  — worker thread
}
```

Plain `#[tokio::test]` uses the current-thread runtime and is unaffected. Under a multi-threaded
runtime, awaiting directly still works — the root future is polled on the test thread — but
anything handed to `tokio::spawn` or `std::thread::spawn` runs the real function.

If a test unexpectedly hits real behaviour, check whether the call crossed a thread boundary
before suspecting the fake. `is_set()` is useful here: assert it on the thread that actually makes
the call.
