struct ClearTarget;

#[fnmock::spyable]
impl ClearTarget {
    fn clear_method(&self, id: i32) {
        let _ = id;
    }

    fn clear_other(&self) {}

    fn clear_generic_method<T: 'static>(&self, a: T) {
        let _ = a;
    }
}

#[test]
fn test_clear_drops_expectations_and_call_history() {
    let spy = ClearTarget::clear_method_spy();
    spy.expect(fnmock::predicate::eq(1)).times(2);
    ClearTarget.clear_method(1);
    ClearTarget.clear_method(1);

    spy.clear();
    spy.assert();

    spy.expect(fnmock::predicate::eq(1)).once();
    ClearTarget.clear_method(1);
    spy.assert();
}

#[test]
fn test_clear_leaves_other_methods_alone() {
    let cleared = ClearTarget::clear_method_spy();
    let other = ClearTarget::clear_other_spy();
    cleared.expect_once();
    other.expect_once();

    cleared.clear();
    ClearTarget.clear_other();

    cleared.assert();
    other.assert();
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_other_method_is_still_checked_after_clear() {
    let cleared = ClearTarget::clear_method_spy();
    let other = ClearTarget::clear_other_spy();
    other.expect_once();

    cleared.clear();

    other.assert();
}

#[test]
fn test_clear_on_generic_method_is_scoped_to_the_instantiation() {
    let spy_string = ClearTarget::clear_generic_method_spy::<String>();
    let spy_i32 = ClearTarget::clear_generic_method_spy::<i32>();
    spy_string.expect_once();
    spy_i32.expect_once();

    spy_string.clear();
    ClearTarget.clear_generic_method(1i32);

    spy_string.assert();
    spy_i32.assert();
}
