//! `Type::method_mock()` carries both halves, for every receiver form.

struct MockTarget {
    base: i32,
}

#[fnmock::mockable]
impl MockTarget {
    fn by_ref(&self, a: i32) -> i32 {
        self.base + a
    }

    fn by_mut_ref(&mut self, a: i32) -> i32 {
        self.base += a;
        self.base
    }

    fn by_value(self, a: i32) -> i32 {
        self.base + a
    }

    fn associated(a: i32) -> i32 {
        a
    }

    fn generic_method<T: 'static>(&self, a: T) -> T {
        a
    }
}

#[test]
fn test_ref_receiver_fake_sees_self_and_expectation_ignores_it() {
    let mock = MockTarget::by_ref_mock();
    mock.setup(|target, a| target.base * 100 + a);
    mock.expect(fnmock::predicate::eq(2)).once();

    assert_eq!(MockTarget { base: 3 }.by_ref(2), 302);

    mock.assert();
}

#[test]
fn test_mut_ref_receiver() {
    let mock = MockTarget::by_mut_ref_mock();
    mock.setup(|target, a| {
        target.base = -1;
        a
    });
    mock.expect(fnmock::predicate::eq(4)).once();

    let mut target = MockTarget { base: 10 };
    assert_eq!(target.by_mut_ref(4), 4);
    assert_eq!(target.base, -1);

    mock.assert();
}

#[test]
fn test_value_receiver() {
    let mock = MockTarget::by_value_mock();
    mock.setup(|target, a| target.base - a);
    mock.expect(fnmock::predicate::eq(1)).once();

    assert_eq!(MockTarget { base: 5 }.by_value(1), 4);

    mock.assert();
}

#[test]
fn test_associated_function() {
    let mock = MockTarget::associated_mock();
    mock.setup(|a| a + 1);
    mock.expect(fnmock::predicate::eq(1)).once();

    assert_eq!(MockTarget::associated(1), 2);

    mock.assert();
}

#[test]
fn test_unfaked_method_runs_real_body_and_is_recorded() {
    let mock = MockTarget::by_ref_mock();
    mock.expect(fnmock::predicate::eq(2)).once();

    assert_eq!(MockTarget { base: 3 }.by_ref(2), 5);

    mock.assert();
}

#[test]
fn test_methods_of_one_impl_are_independent() {
    let by_ref = MockTarget::by_ref_mock();
    let associated = MockTarget::associated_mock();
    by_ref.setup(|_, _| 0);
    by_ref.expect_once();
    associated.expect_never();

    MockTarget { base: 1 }.by_ref(1);

    by_ref.assert();
    associated.assert();
    assert!(!associated.is_set());
}

#[test]
fn test_generic_method_instantiations_are_independent() {
    let mock_i32 = MockTarget::generic_method_mock::<i32>();
    let mock_u8 = MockTarget::generic_method_mock::<u8>();
    mock_i32.setup(|_, a| a + 1);
    mock_i32.expect_once();
    mock_u8.expect_never();

    let target = MockTarget { base: 0 };
    assert_eq!(target.generic_method(1i32), 2);

    mock_i32.assert();
    mock_u8.assert();
}

#[test]
fn test_clear_on_a_method_resets_both_halves() {
    let mock = MockTarget::by_ref_mock();
    mock.setup(|_, _| 0);
    mock.expect_once();

    mock.clear();

    assert!(!mock.is_set());
    mock.assert();
    assert_eq!(MockTarget { base: 3 }.by_ref(2), 5);
}
