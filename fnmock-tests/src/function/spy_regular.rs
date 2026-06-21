fn get_user(id: i32, id2: String) -> String {
    #[cfg(test)]
    let (id, id2) = get_user_spy::GetUserSpyInterface::new().record((id, id2));

    format!("User {} {}", id, id2)
}

#[cfg(test)]
pub(crate) mod get_user_spy {
    use fnmock::spy::{ Predicate, Spy };

    thread_local! {
        static GET_USER_SPY: std::cell::RefCell<Spy<2>> = std::cell::RefCell::new(
            Spy::new("get_user")
        );
    }

    struct GetUserSpyPredicate {
        pred: Box<dyn Fn(&i32, &String) -> bool>,
        display: &'static str,
    }

    impl GetUserSpyPredicate {
        pub fn new(
            pred: impl (Fn(&i32, &String) -> bool) + 'static,
            display: &'static str
        ) -> Self {
            Self { pred: Box::new(pred), display }
        }
    }

    impl Predicate<2> for GetUserSpyPredicate {
        fn evaluate(&self, args: &[Box<dyn std::any::Any>; 2]) -> bool {
            let mut iter = args.iter();
            (self.pred)(
                iter
                    .next()
                    .expect(
                        "The number of arguments in call does not match the number of arguments in expectation."
                    )
                    .downcast_ref::<i32>()
                    .expect("Downcast of params should not fail"),
                iter
                    .next()
                    .expect(
                        "The number of arguments in call does not match the number of arguments in expectation."
                    )
                    .downcast_ref::<String>()
                    .expect("Downcast of params should not fail")
            )
        }

        fn display(&self) -> &'static str {
            self.display
        }
    }

    pub(crate) struct GetUserSpyInterface;

    impl GetUserSpyInterface {
        pub(crate) fn new() -> Self {
            Self
        }

        pub(crate) fn setup(self) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.setup();
            });
            self
        }

        pub(crate) fn record(&self, args: (i32, String)) -> (i32, String) {
            let arr: [Box<dyn std::any::Any>; 2] = [Box::new(args.0), Box::new(args.1)];

            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.record(&arr);
            });

            let mut iter = arr.into_iter();
            (
                *iter
                    .next()
                    .expect(
                        "The number of arguments in call does not match the number of arguments in expectation."
                    )
                    .downcast::<i32>()
                    .expect("Downcast of params should not fail"),
                *iter
                    .next()
                    .expect(
                        "The number of arguments in call does not match the number of arguments in expectation."
                    )
                    .downcast::<String>()
                    .expect("Downcast of params should not fail"),
            )
        }

        pub(crate) fn verify(self) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.verify();
            });
            self
        }

        pub(crate) fn expect_call(
            self,
            expectation: impl (Fn(&i32, &String) -> bool) + 'static,
            expectation_display: &'static str
        ) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_call(
                    Box::new(GetUserSpyPredicate::new(expectation, expectation_display))
                );
            });
            self
        }

        pub(crate) fn expect_call_times(
            self,
            times: usize,
            expectation: impl (Fn(&i32, &String) -> bool) + 'static,
            expectation_display: &'static str
        ) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_call_times(
                    times,
                    Box::new(GetUserSpyPredicate::new(expectation, expectation_display))
                );
            });
            self
        }

        pub(crate) fn expect_call_range<R: std::ops::RangeBounds<usize>>(
            self,
            range: R,
            expectation: impl (Fn(&i32, &String) -> bool) + 'static,
            expectation_display: &'static str
        ) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy: &mut Spy<2>| {
                spy.expect_call_range(
                    range,
                    Box::new(GetUserSpyPredicate::new(expectation, expectation_display))
                );
            });
            self
        }

        pub(crate) fn expect_call_once(
            self,
            expectation: impl (Fn(&i32, &String) -> bool) + 'static,
            expectation_display: &'static str
        ) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_call_once(
                    Box::new(GetUserSpyPredicate::new(expectation, expectation_display))
                );
            });
            self
        }

        pub(crate) fn in_sequence(self) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.in_sequence();
            });
            self
        }

        pub(crate) fn expect_range<R: std::ops::RangeBounds<usize>>(self, range: R) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_range(range);
            });
            self
        }

        pub(crate) fn expect_times(self, times: usize) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_times(times);
            });
            self
        }

        pub(crate) fn expect_never(self) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_never();
            });
            self
        }
    }
}

fn handle_user(id: i32, id2: String) -> String {
    let user = get_user(id, id2);
    user
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_user_spy() {
        let spy = get_user_spy::GetUserSpyInterface
            ::new()
            .setup()
            .expect_call(|i, s| *i == 1 && s == "2", "i == (1, \"2\")")
            .in_sequence();

        let result = handle_user(1, "2".into());

        assert_eq!(result, "User 1 2");

        spy.verify();
    }
}
