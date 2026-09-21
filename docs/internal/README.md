# Internal documentation

These documents describe **how fnmock works on the inside**. They are written for people
changing fnmock itself. If you are looking for how to *use* the crate, start at the
[README](../../README.md), [USAGE.md](../../USAGE.md), [FAKE_FEATURES.md](../FAKE_FEATURES.md),
[SPY_FEATURES.md](../SPY_FEATURES.md) and [LIMITATIONS.md](../LIMITATIONS.md) instead.

## Contents

| Document | What it covers |
| --- | --- |
| [ARCHITECTURE.md](ARCHITECTURE.md) | The three crates, what each one owns, and the shape of the whole system. |
| [TESTING.md](TESTING.md) | Test layout, the trybuild compile-fail suite, CI jobs, and local workflow gotchas. |
| [RELEASE.md](RELEASE.md) | The manual release checklist. |

## Orientation in one minute

- Two attributes exist: `#[fnmock::fakeable]` (replace a body) and `#[fnmock::spyable]`
  (observe a call). Both can be applied to a free function or to an inherent `impl` block.
- Both leave the annotated item where it is, inject a `#[cfg(test)]`-gated statement at the top of
  each body, and generate a `#[cfg(test)]` accessor plus a `#[cfg(test)]` module holding a
  `thread_local!` store.
- All state is thread-local; the test harness gives each `#[test]` its own thread, which is the
  entire isolation story. Nothing is `Send`/`Sync`, and everything uses `Rc`/`RefCell`.
- `fnmock-derive` decides *what code to write*. `fnmock` holds the *runtime that generated code
  calls into*.
