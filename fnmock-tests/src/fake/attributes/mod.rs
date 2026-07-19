/// Regression tests ensuring attributes placed on a `#[fakeable]` item (like `#[deprecated]`) are
/// preserved through macro expansion, regardless of order relative to `#[fnmock::fakeable]` and
/// for both free functions and impl-block methods.
///
/// Each file in `compile_fail/` denies the lint the attribute would normally only warn about, so
/// if fnmock ever dropped the attribute while re-emitting the item, the lint would stop firing and
/// the fixture would compile instead of failing. To regenerate the `.stderr` snapshots after
/// changing an error message, run:
///
/// ```sh
/// TRYBUILD=overwrite cargo test -p fnmock-tests compile_fail_attribute_preservation
/// ```
#[test]
fn compile_fail_attribute_preservation() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/fake/attributes/compile_fail/*.rs");
}
