#[fnmock::spyable]
fn const_generics_multiple<const A: usize, const B: usize>(tag: &str) -> usize {
    let _ = tag;
    A + B
}

#[test]
fn test_const_generics_multiple() {
    let spy = const_generics_multiple_spy::<2, 3>();
    spy.expect(fnmock::predicate::eq("tag".to_string())).once();

    let res = const_generics_multiple::<2, 3>("tag");

    assert_eq!(res, 5);
    spy.assert();
}

#[test]
fn test_const_generics_multiple_are_keyed_positionally() {
    let spy_2_3 = const_generics_multiple_spy::<2, 3>();
    let spy_3_2 = const_generics_multiple_spy::<3, 2>();
    spy_2_3.expect_once();
    spy_3_2.expect_never();

    const_generics_multiple::<2, 3>("tag");

    spy_2_3.assert();
    spy_3_2.assert();
}
