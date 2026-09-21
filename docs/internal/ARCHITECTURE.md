# Architecture

## The workspace

| Crate | Kind | Published | Role |
| --- | --- | --- | --- |
| `fnmock-derive` | proc-macro | yes | Implements `#[fakeable]` and `#[spyable]`. Decides what code is written. |
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
  │  #[fakeable] / #[spyable]     │  emits     │  FakeStore / GenericFakeStore│
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
   that falls through into the real body.
2. **An accessor**, `#[cfg(test)]`-gated: `foo_fake()` / `foo_spy()` for a free function,
   `Type::foo_fake()` / `Type::foo_spy()` for a method. It returns a zero-sized *interface*
   value.
3. **A generated module**, `#[cfg(test)]`-gated, containing the `thread_local!` store, the
   interface struct and its `impl` (the methods a test calls), and — for spies — the matcher type
   and the `internal_record_call` entry point.

Because every generated item carries `#[cfg(test)]`, a release build keeps the original body
verbatim and compiles no fnmock machinery whatsoever.

## State and isolation model

All state lives in `thread_local!` statics inside the generated modules. The runtime types they
hold are single-threaded by construction (`Rc`, `RefCell`, `Rc<dyn Any>`), so nothing is `Send` or
`Sync` and there is no locking anywhere.

The consequences, all of which show up in the user-facing docs, follow directly from that choice:

- Tests are isolated for free — the standard test harness runs each `#[test]` on its own thread,
  so no reset step exists or is needed.
- A fake or spy installed on one thread is invisible on another, so code that crosses a thread
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
