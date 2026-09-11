//! The generic parameter appears only in the return type, so no parameter
//! mentions it. The spy still keys its store by `T`, and the matcher carries it
//! as a phantom.

#[fnmock::spyable]
fn generic_only_in_return<T: Default + 'static>(tag: &str) -> T {
    let _ = tag;
    T::default()
}

#[test]
fn test_generic_only_in_return() {
    let spy = generic_only_in_return_spy::<String>();
    spy.expect(fnmock::predicate::eq("tag".to_string())).once();

    let res: String = generic_only_in_return("tag");

    assert_eq!(res, "");
    spy.assert();
}

#[test]
fn test_instantiations_stay_isolated_when_the_params_are_identical() {
    let spy_string = generic_only_in_return_spy::<String>();
    let spy_u8 = generic_only_in_return_spy::<u8>();
    spy_string.expect_once();
    spy_u8.expect_never();

    let _: String = generic_only_in_return("tag");

    spy_string.assert();
    spy_u8.assert();
}
