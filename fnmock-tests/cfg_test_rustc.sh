#!/bin/sh
# Used as `RUSTC_WORKSPACE_WRAPPER` by the compile-fail tests: cargo calls it as
# `cfg_test_rustc.sh <rustc> <args>...` for the fixture crate only, and it adds
# `--cfg test` so the fixtures see the same expansion as a `cargo test` build.
exec "$@" --cfg test
