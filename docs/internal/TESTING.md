# Testing

Three layers, each answering a different question.

| Layer | Where | Question |
| --- | --- | --- |
| Macro unit tests | `fnmock-derive/src/**` (`#[cfg(test)] mod tests`) | Does each pipeline stage produce the right tokens / the right error? |
| Runtime unit tests | `fnmock/src/**` (`#[cfg(test)] mod tests`) | Do the stores, ranges, expectations and sequences behave? |
| Behavioural tests | `fnmock-tests/src/**` | Does the real attribute, on real code, do the right thing? |

## `fnmock-tests`

A `publish = false` **binary** crate whose `main.rs` does nothing but declare the test modules.
Everything of interest lives in `#[cfg(test)]` unit tests inside it — which it has to, since that
is the only scope where fnmock's accessors exist at all.

```
src/
  common/    features that apply to fakes and spies alike; most files contain a
             `mod fake { .. }` and a `mod spy { .. }` testing the same construct both ways
    params/        by value, by reference, patterns, smart pointers, raw pointers, ...
    generics/      type params, const generics, lifetimes, isolation between instantiations
    impl_block/    receivers, associated functions, visibility, isolation between types
    returns/, traits/, special/, visibility/, attributes/
  fake/      fake-only: captured state, clear/is_set, re-entrancy
  spy/       spy-only: expectations, sequences, generic instantiation scoping
  compile_fail.rs
```

`tokio` is a dependency purely so the thread-isolation tests can show what a multi-thread runtime
does to thread-local state.

The support matrix in [../LIMITATIONS.md](../LIMITATIONS.md) links each cell to the test backing
it, so a new supported (or explicitly unsupported) construct means adding both a test here and a
row there.

## Compile-fail tests

Every file named `*.cf.rs` is a fixture that **must not compile**, paired with a `*.cf.stderr`
snapshot of the exact expected error. `compile_fail.rs` runs them all through
[trybuild](https://docs.rs/trybuild):

```rust
#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/**/*.cf.rs");
}
```

They sit next to the passing tests for the same feature, and most unsupported constructs have a
`_fake` and a `_spy` variant, since the two attributes reject different things.

Regenerating the snapshots after changing an error message:

```sh
TRYBUILD=overwrite cargo test -p fnmock-tests compile_fail
```

Always read the resulting diff — `TRYBUILD=overwrite` will happily bless a *worse* message. These
snapshots are the only place error quality is pinned down.

Because rustc's diagnostics change between releases, compile-fail is **skipped** everywhere but
stable in CI (`-- --skip compile_fail`).

## CI

`.github/workflows/ci.yml`, on pushes to `master` and on every pull request:

| Job | Command |
| --- | --- |
| Rustfmt | `cargo fmt --all --check` |
| Clippy | `cargo clippy --workspace --all-targets --locked -- -D warnings` |
| Test (stable) | `cargo test --workspace --locked` |
| Test (beta) | `cargo test --workspace --locked -- --skip compile_fail` |
| MSRV (1.85) | `cargo check -p fnmock -p fnmock-derive --locked`, then tests with `--skip compile_fail` |
| Docs | `cargo doc -p fnmock -p fnmock-derive --no-deps --locked` with `RUSTDOCFLAGS: -D warnings` |

Note that `fnmock-tests` sets `[lints.clippy] all = "allow"` — the fixtures deliberately contain
odd-looking code — so clippy's `-D warnings` effectively covers the two published crates.

A green `master` is the release gate; see [RELEASE.md](RELEASE.md).
