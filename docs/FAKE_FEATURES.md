# Fake features

A **fake** replaces a function's behaviour: while a fake is installed the real body does not run and
the fake's return value is used instead. For the observe-only counterpart see
[SPY_FEATURES.md](SPY_FEATURES.md).

> For everything fnmock supports and rejects across **both** macros — parameter types and patterns,
> return types, generics, impl blocks, visibility, isolation — see the shared
> **[LIMITATIONS.md](LIMITATIONS.md)** matrix. This document only covers what is unique to fakes.

Every claim links to the test that backs it. Run the whole set with `cargo test -p fnmock-tests`.
Most of these tests live in [`fnmock-tests/src/fake/`](../fnmock-tests/src/fake/); a few illustrate
the accessor shape using the `mod fake` half of a file shared with `#[fnmock::spyable]` under
[`fnmock-tests/src/common/`](../fnmock-tests/src/common/).

## Contents

- [The attribute](#the-attribute)
- [The accessor](#the-accessor)
- [Accessor methods](#accessor-methods)
- [The fake closure](#the-fake-closure)

## The attribute

Apply `#[fnmock::fakeable]` to a free function or an inherent impl block. It:

1. injects a `#[cfg(test)]` lookup at the top of the original body — if a fake is installed it is
   called and its result returned, otherwise the real body runs;
2. generates an accessor named `<fn_name>_fake()`;
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

## The accessor

The accessor is emitted with the same visibility as the function it fakes. Its shape depends on the
item:

| Item | How you call the accessor | Test |
| --- | --- | --- |
| Free function `f` | `f_fake()` | [by_value.rs](../fnmock-tests/src/common/params/by_value.rs) |
| Generic function `f<T>` | `f_fake::<T>()` | [single_generic.rs](../fnmock-tests/src/common/generics/single_generic.rs) |
| Method `m` on `Type` | `Type::m_fake()` | [basic.rs](../fnmock-tests/src/common/impl_block/basic.rs) |
| Method `m` on generic `Type<G>`, itself generic over `M` | `Type::<G>::m_fake::<M>()` | [generic_combined.rs](../fnmock-tests/src/common/impl_block/generics/generic_combined.rs) |

## Accessor methods

| Method | Behaviour | Test |
| --- | --- | --- |
| `setup(closure)` | Install a fake. Calling it again overwrites the previous one. | [clear_and_is_set.rs](../fnmock-tests/src/fake/basic/clear_and_is_set.rs) |
| `clear()` | Remove the fake; later calls run the real implementation again. | [clear_and_is_set.rs](../fnmock-tests/src/fake/basic/clear_and_is_set.rs) |
| `is_set()` | Whether a fake is currently installed. | [clear_and_is_set.rs](../fnmock-tests/src/fake/basic/clear_and_is_set.rs) |

All three take `&self` and return `()`/`bool`, so they do not chain. There is no automatic reset: a
fake stays installed for the rest of the thread's life unless you call `clear()`. In practice the
test harness gives each `#[test]` its own thread, so this only matters when you install fakes
outside a test body.

A fake closure may call back into its own accessor (`is_set`/`setup`/`clear`) without panicking on a
double borrow — the generated code scopes each `RefCell` borrow to a single lookup and hands back an
owned value before the closure runs
([reentrant_fake.rs](../fnmock-tests/src/fake/basic/reentrant_fake.rs)).

## The fake closure

The closure passed to `setup` mirrors the function signature: same parameters, same return type. Two
things follow from that:

- The parameter patterns carry over, so `setup(|(left, right)| …)` destructures a
  `(String, String)` argument the same way the real function does. This is why fakes accept
  destructuring patterns that spies reject — see
  [Parameter patterns](LIMITATIONS.md#parameter-patterns) in LIMITATIONS.md.
  ([tuple_destructuring.rs](../fnmock-tests/src/common/params/patterns/tuple_destructuring.rs))
- The closure has to *produce* a value of the return type, which is why `impl Trait` and `!` are not
  supported in return position even though a spy accepts both — see
  [Return types](LIMITATIONS.md#return-types) in LIMITATIONS.md.

Two further rules:

- For methods the receiver is passed as the **first** closure argument (`|_, a, b|`), and is not
  otherwise matched on or inspected — [basic.rs](../fnmock-tests/src/common/impl_block/basic.rs).
- For `async` functions the closure is a plain **synchronous** closure returning the output type, not
  a future — [async_function.rs](../fnmock-tests/src/common/special/async_function.rs).

For a generic function, each combination of generic arguments gets its own closure and its own
store, so **always spell the arguments out explicitly** on both `setup` and the call site — if the
compiler infers different ones than you expected, the fake silently does not apply and the real body
runs. For a const generic parameter specifically, its value is not passed into the closure at all;
since the fake only applies to that one value, hardcode it
([single_const_generic.rs](../fnmock-tests/src/common/generics/const_generics/single_const_generic.rs)).
See [Generics](LIMITATIONS.md#generics) and
[Restrictions that are not compile errors](LIMITATIONS.md#restrictions-that-are-not-compile-errors)
in LIMITATIONS.md.
