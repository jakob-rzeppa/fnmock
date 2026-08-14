# Features

The user-facing surface of `#[fnmock::fakeable]`: the attribute you apply, the accessor it
generates, and the methods you call to install and manage a fake. For what can and can't be faked —
supported types, patterns, generics, impl blocks, and the isolation/keying rules fakes operate
under — see [CONSTRAINTS.md](CONSTRAINTS.md).

Every entry below is backed by a test in [fnmock-tests/src/fake/](fnmock-tests/src/fake/).

## The attribute

Apply `#[fnmock::fakeable]` to a free function or an inherent impl block. It:

1. injects a `#[cfg(test)]` fake lookup at the top of the original body — if a fake is set, it is
   called and its result returned; otherwise the real body runs,
2. generates an accessor named `<fn_name>_fake()`,
3. generates a hidden module holding the fake in `thread_local!` storage.

```rust
#[fnmock::fakeable]
fn greet(name: String) -> String {
    format!("Real {}", name)
}

#[test]
fn test() {
    greet_fake().setup(|name| format!("Fake {}", name));
    assert_eq!(greet("Test".to_string()), "Fake Test");
}
```

The lookup is `#[cfg(test)]`-gated, so non-test builds carry no fake machinery and no runtime
overhead.

Other attributes you place on the item (`#[deprecated]`, `#[must_use]`, …) are preserved through
expansion — in either order relative to `#[fnmock::fakeable]`, and for both free functions and
impl-block methods. See [attributes/](fnmock-tests/src/fake/attributes/).

## The accessor

The accessor is emitted as `#[cfg(test)]` with the same visibility as the function.

Its exact shape depends on the item being faked:

| Item | How you call the accessor |
| --- | --- |
| Free function `f` | `f_fake()` |
| Generic function `f<T>` | `f_fake::<T>()` |
| Method `m` on `Type` | `Type::m_fake()` |
| Method `m` on generic `Type<G>`, itself generic over `M` | `Type::<G>::m_fake::<M>()` |

For generics the arguments select which fake you get, so each instantiation is faked independently.
Getting those arguments right is a constraint, not a convenience — see
[Generics](CONSTRAINTS.md#generics).

## Accessor methods

| Method | Behaviour | Test |
| --- | --- | --- |
| `setup(closure)` | Install a fake. Calling it again overwrites the previous fake. | [clear_and_is_set.rs](fnmock-tests/src/fake/basic/clear_and_is_set.rs) |
| `clear()` | Remove the fake; subsequent calls run the real implementation again. | [clear_and_is_set.rs](fnmock-tests/src/fake/basic/clear_and_is_set.rs) |
| `is_set()` | Whether a fake is currently installed. | [clear_and_is_set.rs](fnmock-tests/src/fake/basic/clear_and_is_set.rs) |

The closure mirrors the function signature: same parameters, same return type. Parameter patterns
carry over too, so `setup(|(left, right)| …)` destructures a `(String, String)` argument the same
way the real function does (the patterns that are and aren't allowed are listed in
[CONSTRAINTS.md](CONSTRAINTS.md#parameter-patterns)).

- For methods the receiver is passed as the **first** closure argument (`|_, a, b|`).
- For `async` functions the closure is a plain synchronous closure returning the output type — not a
  future.

`setup`/`clear` consume and return `Self`, so calls can be chained
(`foo_fake().setup(...).clear()`) — see
[accessor_chaining.rs](fnmock-tests/src/fake/basic/accessor_chaining.rs).
