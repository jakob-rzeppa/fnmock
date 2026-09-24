# Internal documentation

These documents describe **how fnmock works on the inside**. They are written for people
changing fnmock itself. If you are looking for how to *use* the crate, start at the
[README](../../README.md), [USAGE.md](../../USAGE.md), [FEATURES.md](../FEATURES.md),
[FAKE.md](../FAKE.md), [SPY.md](../SPY.md) and
[LIMITATIONS.md](../LIMITATIONS.md) instead.

## Contents

| Document | What it covers |
| --- | --- |
| [ARCHITECTURE.md](ARCHITECTURE.md) | The three crates, what each one owns, and the shape of the whole system. |
| [TESTING.md](TESTING.md) | Test layout, the trybuild compile-fail suite, CI jobs, and local workflow gotchas. |
| [RELEASE.md](RELEASE.md) | The manual release checklist. |

## Orientation in one minute

- `#[fnmock::mockable]` is the main attribute: replace a body **and** record the call, behind one
  accessor. `#[fnmock::fakeable]` (replace only) and `#[fnmock::spyable]` (record only) are each
  one half of it. Internally, a mock is those two halves composed (see
  [ARCHITECTURE.md](ARCHITECTURE.md)). Each attribute can be applied to a free function or to an
  inherent `impl` block.
- All three leave the annotated item where it is, inject a `#[cfg(test)]`-gated statement at the top of
  each body, and generate a `#[cfg(test)]` accessor plus a `#[cfg(test)]` module holding
  `thread_local!` state (two stores for a mock).
- All state is thread-local; the test harness gives each `#[test]` its own thread, which is the
  entire isolation story. Nothing is `Send`/`Sync`, and everything uses `Rc`/`RefCell`.
- `fnmock-derive` decides *what code to write*. `fnmock` holds the *runtime that generated code
  calls into*.
