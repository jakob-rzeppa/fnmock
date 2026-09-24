# Spies

`#[fnmock::spyable]` is the recording half of a [mock](FEATURES.md) on its own. It records calls so
a test can assert on them, and the real body always runs.

## Example

```rust
#[fnmock::spyable]
fn fetch_user(id: i32) -> String {
    format!("user {id}")
}

#[test]
fn test() {
    let spy = fetch_user_spy();
    spy.expect(fnmock::predicate::eq(2)).once();

    assert_eq!(fetch_user(2), "user 2"); // the real body still runs

    spy.assert();
}
```

The accessor is `<fn_name>_spy()`, shaped like the mock accessor: `f_spy::<T>()` for a generic
function, `Type::m_spy()` for a method.

## Methods

| Method | Behaviour | Test |
| --- | --- | --- |
| `expect(predicate, …)` | Expect calls matching one predicate per parameter. Returns a handle. | [times.rs](../fnmock-tests/src/spy/expectations/times.rs) |
| `expectf(closure)` | The same, but matching with a closure over all parameters. | [expectf.rs](../fnmock-tests/src/spy/expectations/expectf.rs) |
| `expect_times(range)` / `expect_once()` / `expect_never()` | Expect a total call count, regardless of arguments. | [global_times.rs](../fnmock-tests/src/spy/expectations/global_times.rs) |
| `assert()` | Panic unless every expectation on this spy is fulfilled. | [times.rs](../fnmock-tests/src/spy/expectations/times.rs) |
| `clear()` | Drop every expectation, `expect_times` range and recorded call, and remove the spy's steps from any sequence. | [basic.rs](../fnmock-tests/src/spy/clear/basic.rs), [sequences.rs](../fnmock-tests/src/spy/clear/sequences.rs) |

These methods behave exactly as they do on a mock: the expectation handle (`times`, `once`,
`never`, `describe`, `in_sequence`), matching rules and sequences are all the same. See
[Expectations](FEATURES.md#expectations) and [Sequences](FEATURES.md#sequences). Spies and mocks
can share a sequence.

## How it differs from a mock

- **`-> impl Trait` and `-> !` are allowed.** A spy never produces a return value, so it doesn't
  need to name the return type. A mock rejects both, because its closure would have to produce the
  value ([impl_trait_return.rs](../fnmock-tests/src/common/traits/impl_trait_return.rs),
  [never_return_type.rs](../fnmock-tests/src/common/returns/never_return_type.rs)).
- **The body can't be replaced.** There is no `setup` or `is_set`, so every call runs the real
  body.
- **`clear()` has no fake to remove**, so it only resets the expectations and call history.

Everything else works as for a mock: generics, impl blocks and thread isolation. See
[FEATURES.md](FEATURES.md) and the Spies column of [LIMITATIONS.md](LIMITATIONS.md).
