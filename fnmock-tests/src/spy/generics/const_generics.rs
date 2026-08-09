//! Const parameters are keyed by their value, not by the `TypeId` of their
//! type, so `C = 5` and `C = 7` are separate instantiations.

#[fnmock::spyable]
fn const_generics<const C: usize>(a: String) -> String {
    format!("{} {}", a, C)
}

#[test]
fn test_const_generics() {
    let spy = const_generics_spy::<5>();
    spy.expect(fnmock::predicate::eq("Test".to_string())).once();

    let res = const_generics::<5>("Test".to_string());

    assert_eq!(res, "Test 5");
    spy.assert();
}

#[test]
fn test_const_generics_value_isolation() {
    let spy_5 = const_generics_spy::<5>();
    let spy_7 = const_generics_spy::<7>();
    spy_5.expect_once();
    spy_7.expect_never();

    const_generics::<5>("Test".to_string());

    spy_5.assert();
    spy_7.assert();
}
