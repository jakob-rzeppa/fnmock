/// Compile-fail tests for parameter patterns that `#[fnmock::fakeable]` and
/// `#[fnmock::spyable]` explicitly do not support.
///
/// Each file ending in `.cf.rs` is expected to fail
/// compilation with the error message stored in the matching `.cf.stderr` snapshot.
/// To regenerate the snapshots after changing an error message, run:
///
/// ```sh
/// TRYBUILD=overwrite cargo test -p fnmock-tests compile_fail
/// ```
#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/**/*.cf.rs");
}
