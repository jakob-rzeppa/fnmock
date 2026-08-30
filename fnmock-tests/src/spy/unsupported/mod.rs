/// Compile-fail tests for features `#[fnmock::spyable]` explicitly does not support.
///
/// Each file in `compile_fail/` is expected to fail compilation with the error
/// message stored in the matching `.stderr` snapshot. To regenerate the
/// snapshots after changing an error message, run:
///
/// ```sh
/// TRYBUILD=overwrite cargo test -p fnmock-tests compile_fail_unsupported_spy_features
/// ```
#[test]
fn compile_fail_unsupported_spy_features() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/spy/unsupported/compile_fail/*.rs");
}
