# Releasing fnmock

Releases are performed **manually**. There is no automated publish pipeline —
CI only verifies the code. This document is the checklist for cutting a release.

## Overview

- The workspace uses a **single shared version** (`workspace.package.version` in
  the root [`Cargo.toml`](../Cargo.toml)). All crates release together in
  lockstep.
- Two crates are published, in a **forced order** because `fnmock` depends on
  `fnmock-derive`:
  1. `fnmock-derive` (proc-macro)
  2. `fnmock` (main crate)
- `fnmock-tests` is `publish = false` and is **never** published.
- Verification happens through **CI on `master`**, not through manual local
  checks. A release is only published from a green `master`.

## Prerequisites (one-time)

- A [crates.io](https://crates.io) account, authenticated locally:
  ```
  cargo login
  ```
- Owner rights on **both** `fnmock` and `fnmock-derive` on crates.io. For the
  very first publish you own them automatically; afterwards verify with:
  ```
  cargo owner --list fnmock
  cargo owner --list fnmock-derive
  ```
- Push access to the GitHub repository (for the release branch and the tag).

## Release steps

### 1. Cut a release branch

From an up-to-date `master`:

```
git checkout master
git pull
git checkout -b release/vX.Y.Z
```

### 2. Bump the version

The version lives in **one** place, but a dependency pin must be updated in
lockstep:

- Root [`Cargo.toml`](../Cargo.toml) → `[workspace.package]` → `version = "X.Y.Z"`
- [`fnmock/Cargo.toml`](../fnmock/Cargo.toml) → the `fnmock-derive` dependency
  `version = "X.Y.Z"` (this is what crates.io uses once the path is stripped, so
  it **must** match the new version).

Then refresh the lockfile and commit it:

```
cargo build
git add Cargo.toml fnmock/Cargo.toml Cargo.lock
git commit -m "release: vX.Y.Z"
```

### 3. Update the changelog

In [`CHANGELOG.md`](../CHANGELOG.md):

- Move the items under `[Unreleased]` into a new `## [X.Y.Z] - YYYY-MM-DD`
  section.
- Update the comparison links at the bottom of the file.

Commit it (or amend into the release commit).

### 4. Open a PR and merge to `master`

Open a PR from `release/vX.Y.Z` and wait for **CI to pass**. This is the release
gate — the pipeline runs:

- `cargo fmt --all --check` (rustfmt)
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- `cargo test --workspace --locked` on **stable** and **beta**
- MSRV check + tests on **Rust 1.85**
- `cargo doc -p fnmock -p fnmock-derive --no-deps` with `-D warnings`

**Merge with "Squash and merge."** A release branch's only job is the version
bump and changelog update, so squashing gives exactly one clean, tagged commit
on `master` per release.

### 5. Publish from merged `master`

Get the merged result locally, then publish the two crates **in order**.
`fnmock-derive` must be live on crates.io before `fnmock` can be published,
because `fnmock` depends on it.

```
git checkout master
git pull

cargo publish -p fnmock-derive
# wait until fnmock-derive X.Y.Z is indexed on crates.io, then:
cargo publish -p fnmock
```

Notes:

- `cargo publish --dry-run -p fnmock` only works **after** `fnmock-derive
  X.Y.Z` is actually published, since the dry run resolves the dependency from
  crates.io. Dry-running `fnmock-derive` first is always safe.
- If `fnmock` fails to publish because the new `fnmock-derive` isn't indexed
  yet, wait a minute and retry — no other change is needed.

### 6. Tag the release

Tag **`master`'s HEAD** — the commit produced by the merge — then push the tag:

```
git tag vX.Y.Z
git push origin vX.Y.Z
```

> **Do not** tag the pre-merge branch commit. With squash (and rebase) merges,
> the branch commits are rewritten to new SHAs on `master`, so a tag placed on
> the branch would point at a commit that isn't on `master`. Tagging
> `master`'s HEAD after pulling is correct for **any** merge strategy.

### 7. Post-release

- Confirm the docs built on [docs.rs/fnmock](https://docs.rs/fnmock) and
  [docs.rs/fnmock-derive](https://docs.rs/fnmock-derive).
- Delete the release branch.
- Add a fresh `## [Unreleased]` heading to [`CHANGELOG.md`](../CHANGELOG.md) for
  the next cycle (typically as part of the next change, not a separate commit).

## Troubleshooting

- **`fnmock` publish rejected: `fnmock-derive` version not found.**
  The new `fnmock-derive` isn't indexed yet. Wait and retry `cargo publish -p
  fnmock`.
- **Version pin mismatch.** If publishing `fnmock` complains about the
  `fnmock-derive` version, check that the dependency `version` in
  [`fnmock/Cargo.toml`](../fnmock/Cargo.toml) matches the released version.
- **Published a broken release.** You cannot overwrite a published version.
  Yank it so new projects don't pick it up, then release a fixed patch version:
  ```
  cargo yank --version X.Y.Z fnmock
  cargo yank --version X.Y.Z fnmock-derive
  ```
  (Yanking does not delete the version or break existing users; it only
  prevents new dependency resolution.)
