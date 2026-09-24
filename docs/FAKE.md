# Fakes

`#[fnmock::fakeable]` is the replacing half of a [mock](FEATURES.md) on its own. It swaps the body
for a closure and records nothing.

## Example

```rust
#[fnmock::fakeable]
fn greet(name: String) -> String {
    format!("Real {}", name)
}

#[test]
fn test() {
    greet_fake().setup(|name| format!("Fake {}", name));
    assert_eq!(greet("Test".to_string()), "Fake Test");

    greet_fake().clear();
    assert_eq!(greet("Test".to_string()), "Real Test");
}
```

The accessor is `<fn_name>_fake()`, shaped like the mock accessor: `f_fake::<T>()` for a generic
function, `Type::m_fake()` for a method.

## Methods

| Method | Behaviour | Test |
| --- | --- | --- |
| `setup(closure)` | Install a fake. Calling it again replaces the previous one. | [clear_and_is_set.rs](../fnmock-tests/src/fake/clear_and_is_set.rs) |
| `is_set()` | Whether a fake is currently installed. | [clear_and_is_set.rs](../fnmock-tests/src/fake/clear_and_is_set.rs) |
| `clear()` | Remove the fake; later calls run the real body again. | [clear_and_is_set.rs](../fnmock-tests/src/fake/clear_and_is_set.rs) |

## How it differs from a mock

- **Destructuring parameters are allowed.** `(left, right): (String, String)` and
  `[a, b]: [i32; 2]` compile, because the closure repeats the pattern: `setup(|(left, right)| …)`.
  A mock rejects them, because it has no name to record such a parameter under
  ([tuple_destructuring.rs](../fnmock-tests/src/common/params/patterns/tuple_destructuring.rs),
  [slice_destructuring.rs](../fnmock-tests/src/common/params/patterns/slice_destructuring.rs)).
- **Nothing is recorded.** There is no `expect` or `assert`, and a fake can't join a `Sequence`.
- **`clear()` only removes the fake.**

Everything else works as for a mock: the rules for the closure, generics, impl blocks and thread
isolation. See [FEATURES.md](FEATURES.md) and the Fakes column of [LIMITATIONS.md](LIMITATIONS.md).
