/// # Compile-fail tests
///
/// Each file ending in `.cf.rs` is expected to fail
/// compilation with the error message stored in the matching `.cf.stderr` snapshot.
/// To regenerate the snapshots after changing an error message, run:
///
/// ```sh
/// TRYBUILD=overwrite cargo test -p fnmock-tests compile_fail
/// ```
///
/// The fixtures are compiled with `--cfg test` (added by `cfg_test_rustc.sh`), since everything
/// the macros generate is behind `#[cfg(test)]`.
#[test]
fn compile_fail() {
    let wrapper = concat!(env!("CARGO_MANIFEST_DIR"), "/cfg_test_rustc.sh");
    // SAFETY: no other test in this binary reads or writes the environment.
    unsafe { std::env::set_var("RUSTC_WORKSPACE_WRAPPER", wrapper) };

    let t = trybuild::TestCases::new();
    t.compile_fail("src/**/*.cf.rs");
}
