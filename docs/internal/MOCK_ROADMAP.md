# Mocks — design and roadmap

A working document for the next feature: `#[fnmock::mockable]`, a **mock** = a fake and a spy on
the same item, behind one accessor. Design agreed 2026-09-22; nothing below is implemented yet.

This is an internal planning document. It is deleted or folded into
[ARCHITECTURE.md](ARCHITECTURE.md) once the feature ships.

## Contents

- [The surface](#the-surface)
- [What an expansion looks like](#what-an-expansion-looks-like)
- [`clear()`](#clear)
- [Semantics](#semantics)
- [Pipeline changes](#pipeline-changes)
- [Rejections and error wording](#rejections-and-error-wording)
- [Testing](#testing)
- [Documentation](#documentation)
- [Implementation order](#implementation-order)
- [Risks](#risks)

## The surface

A third attribute, `#[fnmock::mockable]`, alongside `#[fakeable]` and `#[spyable]`, applicable to
the same two things: a free function and an inherent impl block.

```rust
#[fnmock::mockable]
fn fetch_user_name(id: u32) -> String {
    // real database call
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

One accessor, `fetch_user_name_mock()`, returning one zero-sized interface value that carries
**both** halves' methods flat:

| From the fake half | From the spy half | Mock-only |
| --- | --- | --- |
| `setup(closure)` | `expect(pred, ..)` / `expectf(closure)` | `clear()` |
| `is_set()` | `expect_times(n)` / `expect_once()` / `expect_never()` | |
| | `assert()` | |

The two method sets overlap in exactly one name. Both halves define `clear()` — the fake's removes
the installed fake, the spy's drops expectations and call history — and two inherent methods of the
same name on one struct do not compile. The mock resolves it by **suppressing both halves' `clear()`
and emitting one of its own that does both** (see [`clear()`](#clear)). Everything else merges flat.
`#[mockable]` does **not** additionally emit `_fake()` / `_spy()` accessors — the merged interface
covers every call.

Stacking `#[fakeable]` + `#[spyable]` on one item is **not** the supported route and is not made to
work: the fake's early return would run before the spy's record statement, so faked calls would go
unrecorded.

## What an expansion looks like

One module, one accessor, one interface struct, two stores:

```rust
fn get_user(id: u32) -> String {
    #[cfg(test)]
    {
        self::get_user_mock_module::internal_record_call(&id);        // spy half
        if let Some(implementation) = self::get_user_mock_module::implementation() {
            return implementation(id);                                // fake half
        }
    }
    /* original body */
}

#[cfg(test)]
fn get_user_mock() -> self::get_user_mock_module::GetUserMockInterface {
    self::get_user_mock_module::interface()
}

#[cfg(test)]
mod get_user_mock_module {
    use super::*;

    pub struct GetUserMockInterface;                    // shared builder, emitted once

    thread_local! { static GET_USER_FAKE_STORE: ... }   // fake parts
    pub(super) fn implementation() -> Option<...> { ... }
    impl GetUserMockInterface { /* setup, is_set */ }   // fake's clear suppressed

    thread_local! { static GET_USER_SPY_STORE: ... }    // spy parts
    pub struct GetUserMatcher { ... }
    impl GetUserMockInterface { /* expect, expectf, expect_times, ..., assert */ }  // spy's clear suppressed
    pub(super) fn internal_record_call(id: &u32) { ... }

    impl GetUserMockInterface { /* clear: fake + expectations */ }   // mock-only

    pub(super) fn interface() -> GetUserMockInterface { ... }  // shared builder, once
}
```

Several inherent `impl` blocks on one struct are legal Rust and the methods merge, provided no name
repeats. Both existing `build_interface_impl` builders are already parameterized by interface name,
so handing both the same `GetUserMockInterface` merges everything except `clear()`, which needs the
small piece of new codegen described under [`clear()`](#clear).

### Names

Only the three outward-facing names get a mock flavour, from a new
`scheme/{function,impl_block}/mock/names.rs`:

| | Free function | Impl block method |
| --- | --- | --- |
| Module | `get_user_mock_module` | `user_service__get_user_mock_module` |
| Accessor | `get_user_mock()` | `UserService::get_user_mock()` |
| Interface | `GetUserMockInterface` | `UserServiceGetUserMockInterface` |

The half-internal names — `GET_USER_FAKE_STORE`, `GET_USER_SPY_STORE`, `GetUserMatcher`,
`GetUserMatcherParams` — are reused **verbatim** from the existing builders. They live inside a
private module, so keeping "fake" / "spy" in them collides with nothing and means the half name
builders need no flavour parameter at all.

### The one collision

Both generated modules define `pub(super) fn interface()`. Resolved by moving
`build_interface_struct` and `build_interface_getter` out of
`expandable/common/{fake,spy}/module/` into `expandable/common/interface/`, called once per module.

The two copies are **byte-identical today**, so this is a pure dedupe: fake and spy expansions come
out token-for-token unchanged and no existing test moves.

## `clear()`

On a mock, `clear()` resets **the whole mock**: it removes the installed fake *and* drops every
expectation, global `expect_times` range and recorded call, so the accessor is back to a
freshly-created state. After `clear()`, `is_set()` is `false`, calls run the real body, and
`assert()` passes vacuously.

```rust
let mock = get_user_mock();
mock.setup(|_| "Test".into());
mock.expect(eq(1)).once();

get_user(1);
mock.clear();                 // fake gone, expectation gone, call history gone

get_user(2);                  // real body; recorded afresh, no expectation to violate
mock.assert();                // passes
```

Because a mock records every call, clearing only the fake would leave stale expectations to fail a
later `assert()`, and clearing only the spy would leave the fake answering — neither is the reset a
caller means by "clear the mock". There is no half-clear on the mock interface; a test that wants
one uses a plain `#[fakeable]` / `#[spyable]`.

Generics follow the existing per-instantiation rule: `get_user_mock::<i32>().clear()` resets both
halves for `i32` and leaves `::<u8>()` untouched on both sides.

### Codegen

Both halves' `build_interface_impl` gain an `include_clear: bool`. `#[fakeable]` / `#[spyable]` pass
`true` and expand token-for-token as before; the mock passes `false` for both. A new
`expandable/common/mock/clear.rs` builds the one combined method, taking both store names and the
`GenericScheme`, in its own `impl` block:

```rust
pub fn clear(&self) {
    GET_USER_FAKE_STORE.with_borrow_mut(|fake| fake.clear_for([/* keys */]));   // fake half
    GET_USER_SPY_STORE.with_borrow_mut(|store| store.clear_for(&[/* keys */])); // spy half
}
```

The two statements are separate borrows of separate thread-locals, so a fake closure that calls
`clear()` on its own mock (the existing re-entrancy case) does not double-borrow. The non-generic
and `#[fakeable]`-only forms of the fake store use the same `clear` the fake half emits today; the
mock builder branches on `GenericScheme` exactly as the two existing builders do. No runtime-crate
change: `clear_for` and `SpyStore::clear` already exist.

## Semantics

**Record first, then the fake.** The spy half observes every call, whether or not a fake intercepts
it. This is the point of a mock — return a canned value *and* assert how it was asked for. Checking
the fake first would make expectations silently stop matching the moment a fake is installed.

The order is also what makes the injected block borrow-check: `internal_record_call(&id)` takes
shared references whose borrows end at that statement, and the fake's `implementation(id)` then
moves the same parameters.

Everything else falls out of the two halves being independent stores:

| Topic | Behaviour |
| --- | --- |
| Generics | Both halves derive from the same `GenericScheme`, and `GenericFakeStore` / `GenericSpyStore` key on the same `GenericKeyPart`s. `get_user_mock::<i32>()` controls the fake *and* the expectations for that instantiation; `::<u8>()` is untouched on both sides. |
| Sequences | The spy half is a spy: it joins `Sequence`s and interleaves with plain spies and other mocks. |
| `supports_expect == false` | A parameter type that still names a lifetime yields `expectf` but not `expect`, exactly as a spy does today. No mock-specific handling. |
| Receivers | The fake is handed the receiver as its first closure argument; the spy does not record it. Both stay true in a mock: `setup` sees `&self`, expectations never match on it. |
| `clear()` | One method that clears both halves — see [`clear()`](#clear). |
| Isolation, re-entrancy, unexpected calls | Unchanged. Both stores are `thread_local!`s in the same module; a fake that calls back into the mocked function records again. |

The consequence to document: an intercepted call means the real body never runs, so its side
effects are skipped — same as a plain fake, but easier to forget when also asserting on calls.

## Pipeline changes

Approach: **compose at the scheme layer, assemble at the expandable layer.**

Rejected alternatives: running both strategies to completion and merging two `FunctionExpanded`s
(each produces its own mutated copy of the user's function, and `OriginalFn` is deliberately not
`Clone` — merging would have to happen on token streams, the untyped mush the staged pipeline
exists to prevent); and generalizing `Strategy` into N composable halves (a trait for arbitrary
composition when there are exactly two halves and one composition).

### `item_info/`, `expanded/`, and the `fnmock` runtime crate: no changes

`FunctionExpandable` / `ImplMethodExpandable` already carry `module_parts: Vec<TokenStream>` and a
single `inline_call: syn::Block` — exactly the shape a mock needs. The runtime is untouched: the
two stores are independent thread-locals and `Sequence`, `Expectation`, `CallRange` and the matcher
machinery are unaffected.

### `scheme/` — split each half into its own struct

```rust
// scheme/function/fake/mod.rs
pub struct FakeScheme { store_name, fn_closure_trait, fake_call_values }
pub fn build_fake_scheme(info: &FunctionInfo) -> syn::Result<FakeScheme>

pub struct FunctionFakeScheme { common: FunctionCommonScheme, fake: FakeScheme }
```

and `build_spy_scheme(&FunctionInfo) -> syn::Result<SpyScheme>` likewise, with `SpyScheme` holding
`store_name`, `matcher_name`, `params_name`, `param_idents`, `param_types`, `params_tuple_types`,
`reference_call_values`, `generic_display_fragments` and `supports_expect`.

Taking `&FunctionInfo` is the whole trick. Today's `TryFrom` consumes `FunctionInfo` only to move
`original` out; keeping that move in the flavour's own `TryFrom` lets both halves borrow, so
`FunctionInfo` never needs `Clone`.

```rust
impl TryFrom<FunctionInfo> for FunctionMockScheme {
    fn try_from(info: FunctionInfo) -> syn::Result<Self> {
        let fake = build_fake_scheme(&info)?;     // first error wins
        let spy = build_spy_scheme(&info)?;
        let module_name = mock::names::build_module_name(&info.name);
        let accessor_name = mock::names::build_accessor_name(&info.name);
        let interface_name = mock::names::build_interface_name(&info.name);
        let display_name = info.name.to_string();
        let generic_scheme = build_generic_scheme(&info.generic_params);

        Ok(Self {
            common: FunctionCommonScheme {
                vis: info.visibility,
                original: info.original,
                module_name, display_name, accessor_name, interface_name, generic_scheme,
            },
            fake,
            spy,
        })
    }
}
```

Names are computed **before** the struct literal so nothing reads `info` after the partial move.
The same shape applies to `ImplMockScheme` over `ImplCommonMethodScheme`.

The three flavour `TryFrom`s end up near-identical apart from which names module they call. Leave
them as three explicit impls; a name-provider trait over a closed set of three flavours costs more
than it saves.

### `expandable/` — extract the assembly lists

```rust
// expandable/common/fake/module/mod.rs
pub fn build_module_parts(
    display_name: &str,
    interface_name: &syn::Ident,
    generic_scheme: Option<&GenericScheme>,
    fake: &FakeScheme,
) -> Vec<TokenStream>               // [store, implementation_getter, interface_impl]
```

and the spy twin returning `[store, matcher, interface_impl, record_call]`. Plain arguments rather
than a shared trait, because `FunctionCommonScheme` and `ImplCommonMethodScheme` only overlap on a
subset.

Note what is **absent** from both lists: `interface_struct` and `interface_getter`, which the
caller emits once per module. Both `interface_impl` entries take `include_clear`; the function-level
builders default it to `true`, so single-half callers are unchanged.

All six `TryFrom<..Scheme> for ..Expandable` impls (function / impl × fake / spy / mock) then
collapse to the same three lines:

```rust
module_parts: [interface_struct] + fake_parts + spy_parts + [mock_clear] + [interface_getter],
inline_call:  merge(spy_inline, fake_inline),   // stmts concatenated, spy first
```

with the single-half flavours passing only their own half (and `include_clear: true`); `mock_clear`
exists only in the mock flavour. `merge` concatenates two
`syn::Block`s' statements; it lives in `expandable/common/`.

This deletes real duplication: `expandable/function/fake/mod.rs` and
`expandable/impl_block/fake/mod.rs` build identical five-part lists today, as do the two spy files
with their six-part lists.

### `strategy/` and the entry point

- `strategy/{function,impl_block}/mock.rs` — twelve lines each, same as the existing four.
- `mockable.rs` mirroring `fakeable.rs`.
- `#[proc_macro_attribute] pub fn mockable` in `lib.rs`, with a rustdoc table covering both halves'
  methods.
- `fnmock/src/lib.rs` needs no edit: line 103 is `pub use fnmock_derive::*;`.

Once the entry-point error message is attribute-agnostic (below), `fakeable.rs`, `spyable.rs` and
`mockable.rs` differ only in which two strategies they dispatch to, so they collapse to one generic
helper plus three one-line callers:

```rust
// entry.rs
pub fn dispatch<F: Strategy<Item = syn::ItemFn>, I: Strategy<Item = syn::ItemImpl>>(
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream>
```

### Expansion-equivalence gate

Because the extracted builders are called with the same arguments in the same order, `#[fakeable]`
and `#[spyable]` must expand **token-for-token** as they do today. Every existing macro unit test
and every `.cf.stderr` snapshot passes untouched. That is the refactor's acceptance criterion,
verified before any mock code is written.

## Rejections and error wording

A mock is the **intersection** of what the two halves support. Both halves run; the first error
wins, fake checked first. The four rows where the halves disagree today all become compile errors
under `#[mockable]`:

| Construct | `#[fakeable]` | `#[spyable]` | `#[mockable]` |
| --- | --- | --- | --- |
| `-> impl Trait` | ❌ | ✅ | ❌ (fake's message) |
| `-> !` | ❌ | ✅ | ❌ (fake's message) |
| Tuple destructuring param | ✅ | ❌ | ❌ (spy's message) |
| Slice / array destructuring param | ✅ | ❌ | ❌ (spy's message) |

Each half's existing spanned message is reused verbatim — no new error-message code.

### Wording changes

Three messages name an attribute and would read wrong under a third one. They are reworded to
describe the mechanic instead, matching the voice already used elsewhere ("The macro does not
support const fn…"):

| Site | Now | Becomes |
| --- | --- | --- |
| `fakeable.rs:14`, `spyable.rs:14` | "The `#[fakeable]` / `#[spyable]` attribute can only be applied to functions and impl blocks." | "The macro can only be applied to functions and impl blocks." |
| `scheme/common/spy_param.rs:187` | "The `#[spyable]` attribute only supports plain identifier parameters. This parameter destructures its value, so there is no name to match it under." | "Recording a call needs a name for each parameter. This parameter destructures its value, so there is no name to match it under." |

No other message mentions an attribute. Exactly four `.cf.stderr` snapshots quote these strings:

- `fnmock-tests/src/common/impl_block/special/unsupported_on_struct_fake.cf.stderr`
- `fnmock-tests/src/common/impl_block/special/unsupported_on_struct_spy.cf.stderr`
- `fnmock-tests/src/common/params/patterns/unsupported_slice_destructuring_spy.cf.stderr`
- `fnmock-tests/src/common/params/patterns/unsupported_tuple_destructuring_spy.cf.stderr`

Re-bless with `TRYBUILD=overwrite` and read the diff by hand, per
[TESTING.md](TESTING.md). The remaining 54 fixtures do not move.

## Testing

### `common/` gets a third arm

121 of its files already carry `mod fake` / `mod spy`; each gains a `mod mock` running the same
construct through `#[fnmock::mockable]`, exercising both halves where it is cheap (`setup` +
`expect` + `assert`). Two files are shaped differently and get equivalent treatment:
`common/visibility/same_name_isolation.rs` (fake-only, nested `mod first` / `mod second`) and
`common/params/reference_in_container.rs` (flat, fake-only).

The four divergent constructs get a `*_mock.cf.rs` fixture **instead of** a `mod mock`:

| Construct | Today | Adds |
| --- | --- | --- |
| Tuple destructuring param | `mod fake` ✅ + `.._spy.cf.rs` | `unsupported_tuple_destructuring_mock.cf.rs` |
| Slice / array destructuring param | `mod fake` ✅ + `.._spy.cf.rs` | `unsupported_slice_destructuring_mock.cf.rs` |
| `-> impl Trait` | `mod spy` ✅ + `.._fake.cf.rs` | `unsupported_impl_trait_return_mock.cf.rs` |
| `-> !` | `mod spy` ✅ + `.._fake.cf.rs` | `unsupported_never_return_type_mock.cf.rs` |

A fifth fixture pins a shared rejection under the new attribute: `const fn` with `#[mockable]`,
which fails in `item_info` before either half runs.

### `fake/` and `spy/` get a second arm

Both directories adopt `common/`'s convention. Each file's contents are wrapped in `mod fake { .. }`
(resp. `mod spy { .. }`) and duplicated into `mod mock { .. }` with `#[fnmock::mockable]` and the
`_mock()` accessor in place of `_fake()` / `_spy()`. Bodies otherwise unchanged.

- `fake/` — 4 files: `clear_and_is_set`, `generic_clear_and_is_set`, `captured_state`,
  `reentrant_fake`. The two `clear` files keep their fake assertions in the mock arm; that `clear()`
  also resets expectations is pinned in `mock/clear.rs`, not here.
- `spy/` — 27 files: `expectations/` (8), `sequences/` (13), `generics/` (4), `lifetimes/` (1).

All 31 targets take plain-identifier parameters and return nameable types, so all are inside the
intersection. The `expectf`-only arms (where a `Ref<'_>` parameter makes `supports_expect` false)
copy across unchanged, since the spy tests already use `expectf` there.

If a target turns out to sit outside the intersection, its mock arm becomes a one-line comment
naming the [LIMITATIONS.md](../LIMITATIONS.md) row instead of a module.

### `mock/` — interactions only

The things neither half can test alone:

| File | Asserts |
| --- | --- |
| `record_then_fake.rs` | a faked call is still recorded, expectations match it, and the real body's side effects are skipped |
| `merged_interface.rs` | `setup` and `expect` on the same accessor value; a faked call is recorded and matched before and after further `setup` calls |
| `clear.rs` | `clear()` removes the fake (`is_set()` false, real body runs) **and** drops expectations, global `expect_times` and call history (`assert()` passes after a violated expectation is cleared); a fake calling `clear()` on its own mock does not panic; a cleared mock can be set up and expected on again |
| `generics.rs` | `foo_mock::<i32>()` moves both halves together, including `clear()`; `::<u8>()` untouched on both sides |
| `impl_block.rs` | `Type::method_mock()` with both halves, receiver forms, associated functions |
| `sequences.rs` | a mock interleaves with a plain spy in one `Sequence` |

### Cost

Roughly 1.5× the behavioural suite (~150 new test modules) and its compile time, which matters
because `fnmock-tests` is macro-expansion-heavy. The work splits cleanly by subtree (`params/`,
`generics/`, `impl_block/`, `returns/`, `traits/`, `special/`, `visibility/`, `attributes/`) across
several commits.

## Documentation

| Doc | Change |
| --- | --- |
| `docs/MOCK_FEATURES.md` | New, mirroring the other two feature docs: the attribute, the `_mock()` accessor, the merged method table, `clear()` resetting both halves, the record-then-fake rule, and the skipped-side-effects caveat. |
| `docs/LIMITATIONS.md` | A real third column. Every row links to its `mod mock` arm, or to the `_mock.cf.rs` fixture where the intersection bites — the doc's contract that every cell links to a test is preserved. The intro ("The two macros mostly agree…") is rewritten for three, stating the intersection rule. |
| `README.md` | "Mocks" leaves Work in Progress; a third example joins the fake and spy ones. |
| `USAGE.md` | The `_mock()` accessor. |
| `docs/internal/ARCHITECTURE.md` | Three attributes; the shared interface builders; a mock module holds two stores. |
| `docs/internal/TESTING.md` | The new arms and the `mock/` tree. |

## Implementation order

1. **Refactor, no behaviour change.** Split the schemes, extract `build_module_parts`, move the
   interface builders, add `include_clear` (defaulting to `true`), add `entry.rs`, reword the three messages, re-bless the four snapshots.
   Gate: full suite green and fake / spy expansions token-identical.
2. **Tests first.** Every `mod mock` arm, the `mock/` directory, and the five compile-fail
   fixtures — written and failing. Then stop and confirm before any codegen.
3. **Mock codegen.** Names module, the `include_clear` flag and `mock/clear.rs` builder,
   `FunctionMockScheme` / `ImplMockScheme`, the two mock `TryFrom`s, the two strategies, `mockable.rs`, the `lib.rs` attribute.
4. **Docs.**
5. **`0.3.0`** as its own step, per [RELEASE.md](RELEASE.md).

## Risks

- **Lifetime parameters in the fake half.** `spy/lifetimes/` and the `Ref<'_>` targets require the
  fake's closure bound to bind those lifetimes higher-ranked. Believed supported; step 2 finds out
  before any codegen is written, which is the right time.
- **`clear()` semantics surprise.** A mock's `clear()` is broader than a fake's. Someone porting a
  `#[fakeable]` test that calls `clear()` to swap fakes mid-test will also wipe their expectations.
  Called out in `MOCK_FEATURES.md`; `setup` alone already replaces a fake without clearing.
- **Snapshot churn.** `TRYBUILD=overwrite` will happily bless a worse message. Read every diff.
- **Suite size.** Step 2 is the largest single diff in the project's history. Split it by subtree.
