/// Compile-fail tests for parameter patterns that `#[fnmock::fakeable]` and
/// `#[fnmock::spyable]` explicitly do not support.
///
/// Each file ending with `.unsupported.{fake/spy}.rs` is expected to fail
/// compilation with the error message stored in the matching `.stderr` snapshot.
/// To regenerate the snapshots after changing an error message, run:
///
/// ```sh
/// TRYBUILD=overwrite cargo test -p fnmock-tests compile_fail_unsupported
/// ```
#[test]
fn compile_fail_unsupported() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/**/*.unsupported.fake.rs");
    t.compile_fail("src/**/*.unsupported.spy.rs");
}
