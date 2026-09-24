# Architecture

## The workspace

| Crate | Kind | Published | Role |
| --- | --- | --- | --- |
| `fnmock-derive` | proc-macro | yes | Implements `#[fakeable]`, `#[spyable]` and `#[mockable]`. Decides what code is written. |
| `fnmock` | library | yes | The runtime the generated code calls into, plus the re-exports a test touches. |
| `fnmock-tests` | binary | **no** (`publish = false`) | Behavioural test suite: real macro invocations compiled and run, plus the trybuild compile-fail snapshots. |

The workspace uses one shared version (`workspace.package.version`).

so users only ever add `fnmock` to `[dependencies]`. Note that fnmock is a **regular** dependency,
not a dev-dependency: the attribute is applied to production code.

## The two halves

```
                    compile time                          test run time
  ┌───────────────────────────────┐            ┌──────────────────────────────┐
  │        fnmock-derive          │            │            fnmock            │
  │                               │            │                              │
  │  fakeable/spyable/mockable    │  emits     │  FakeStore / GenericFakeStore│
  │  ──────────────────────────▶  │ ─────────▶ │  SpyStore  / GenericSpyStore │
  │  item ▸ info ▸ scheme ▸       │  code      │  Matcher, Expectation,       │
  │  expandable ▸ expanded        │  naming    │  ExpectationHandle, Sequence,│
  │                               │  these     │  CallRange                   │
  └───────────────────────────────┘            └──────────────────────────────┘
```

The macro crate never links against the runtime crate — it only writes absolute paths like
`::fnmock::spy_store::SpyStore` into token streams. That coupling is untyped, so a rename in
`fnmock` that is not mirrored in `fnmock-derive` fails in `fnmock-tests`, not at the macro
crate's own build.

## What an expansion looks like

Every expansion produces three things (four for an impl block, where accessors are collected into
one `impl` and there is one module per method):

1. **The original item, with an injected statement** at the top of each body, wrapped in
   `#[cfg(test)]`. For a fake it is an early return; for a spy it is a call-recording statement
   that falls through into the real body; for a mock it is both, the recording first and the early
   return second.
2. **An accessor**, `#[cfg(test)]`-gated: `foo_fake()` / `foo_spy()` / `foo_mock()` for a free
   function, `Type::foo_fake()` / `Type::foo_spy()` / `Type::foo_mock()` for a method. It returns a zero-sized *interface*
   value.
3. **A generated module**, `#[cfg(test)]`-gated, containing the `thread_local!` store, the
   interface struct and its `impl` (the methods a test calls), and — for spies and mocks — the
   matcher type and the `internal_record_call` entry point.

Because every generated item carries `#[cfg(test)]`, a release build keeps the original body
verbatim and compiles no fnmock machinery whatsoever.

## State and isolation model

All state lives in `thread_local!` statics inside the generated modules. The runtime types they
hold are single-threaded by construction (`Rc`, `RefCell`, `Rc<dyn Any>`), so nothing is `Send` or
`Sync` and there is no locking anywhere.

The consequences, all of which show up in the user-facing docs, follow directly from that choice:

- Tests are isolated for free — the standard test harness runs each `#[test]` on its own thread,
  so no reset step exists or is needed.
- A fake, spy or mock installed on one thread is invisible on another, so code that crosses a thread
  boundary (`std::thread::spawn`, a multi-thread `tokio` runtime) silently runs the real body.
  `is_set()` exists largely so a test can catch that case.
- A fake closure is stored as `Rc<dyn Fn(..)>` and is therefore free to capture non-`Send` state.

## Unit of storage

Each annotated **function** (or **method**) gets its own store, its own module and its own
interface — there is no global registry, and nothing is keyed by function name at runtime. The
name that reaches the runtime (`FakeStore::new("fetch_user_name")`) is only a label for panic
messages.

Generic items are the one exception: a single store holds one entry **per combination of generic
arguments**, keyed by `GenericKeyPart`s built from the call's own type and const parameters. That
is why type parameters must be `'static` (they are keyed by `TypeId`) and why const parameters are
keyed by value.

A mock is the one item with **two** stores: `FOO_FAKE_STORE` and `FOO_SPY_STORE` sit side by side
in the same module and are independent thread-locals. Nothing links them at runtime.

## Fakes, spies and mocks share their halves

`#[mockable]` is not a third implementation. A mock is a fake and a spy composed, and the pipeline
is arranged so that each half exists exactly once:

- **`scheme/`** splits each half into its own struct, `FakeScheme` and `SpyScheme`, built by
  `build_fake_scheme` / `build_spy_scheme` from a *borrowed* `FunctionInfo` (or the impl-method
  equivalents). Each flavour's `TryFrom` (`FunctionFakeScheme`, `FunctionSpyScheme`,
  `FunctionMockScheme`, and the `Impl*` twins) pairs a common part with the halves it needs. A
  mock builds both, fake first, so a construct one half rejects is a compile error and the first
  error wins — a mock supports the **intersection** of the two halves. Only the outward-facing
  names (module, accessor, interface) have a mock flavour, in `scheme/mock/*/names.rs`; the store,
  matcher and params names are the halves' own and are reused verbatim.
- **`expandable/common/{fake,spy}/module/module_parts.rs`** each build one half's module parts. The
  single-half expandables call one of them; the mock's `expandable/common/mock/module_parts.rs`
  calls both.
- **The interface struct and getter** (`expandable/common/interface/`) are emitted **once per
  module**, by the caller, because both halves' generated modules would otherwise each define
  `interface()`. Both halves' `impl` blocks then attach to the same interface struct; several
  inherent `impl` blocks on one type are legal as long as no method name repeats.
- **`clear()` is the one name both halves define**, so both `build_interface_impl`s take an
  `include_clear` flag. `#[fakeable]` and `#[spyable]` pass `true`; the mock passes `false` for
  both and emits one `clear` of its own (`expandable/common/mock/clear.rs`) that resets both
  stores in two separate borrows, so a fake closure that calls `clear()` on its own mock does not
  double-borrow.
- **The injected block** is the spy's recording statement followed by the fake's early return
  (`expandable/common/mock/inline_call.rs` concatenates the two blocks' statements). Recording
  first is what makes a faked call observable, and it is also what makes the block borrow-check:
  `internal_record_call` takes shared references that end at that statement, and the fake's
  `implementation(..)` then moves the same parameters.
- **`entry.rs`** holds the one `dispatch` that turns a parsed item into either the function or the
  impl-block strategy, so `fakeable.rs`, `spyable.rs` and `mockable.rs` differ only in which two
  strategies they name.

The runtime crate is untouched by mocks: the two stores are ordinary `FakeStore` / `SpyStore`s.
