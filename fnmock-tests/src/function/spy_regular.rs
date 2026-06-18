fn get_user(id: i32, id2: String) -> String {
    #[cfg(test)]
    let (id, id2) = get_user_spy::GetUserSpyInterface::new().record((id, id2));

    format!("User {} {}", id, id2)
}

#[cfg(test)]
pub(crate) mod get_user_spy {
    use fnmock::spy::Spy;

    thread_local! {
        static GET_USER_SPY: std::cell::RefCell<Spy<(i32, String)>> = std::cell::RefCell::new(
            Spy::new("get_user")
        );
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
            GET_USER_SPY.with_borrow_mut(|spy| { spy.record(args) })
        }

        pub(crate) fn verify(self) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.verify();
            });
            self
        }

        pub(crate) fn expect_call(
            self,
            expectation: fn(&(i32, String)) -> bool,
            expectation_display: &'static str
        ) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_call(expectation, expectation_display);
            });
            self
        }

        pub(crate) fn expect_call_times(
            self,
            times: usize,
            expectation: fn(&(i32, String)) -> bool,
            expectation_display: &'static str
        ) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_call_times(times, expectation, expectation_display);
            });
            self
        }

        pub(crate) fn expect_call_range<R: std::ops::RangeBounds<usize>>(
            self,
            range: R,
            expectation: fn(&(i32, String)) -> bool,
            expectation_display: &'static str
        ) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_call_range(range, expectation, expectation_display);
            });
            self
        }

        pub(crate) fn expect_call_once(
            self,
            expectation: fn(&(i32, String)) -> bool,
            expectation_display: &'static str
        ) -> Self {
            GET_USER_SPY.with_borrow_mut(|spy| {
                spy.expect_call_once(expectation, expectation_display);
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
            .expect_call(|i| *i == (1, "2".into()), "i == (1, \"2\")")
            .in_sequence();

        let result = handle_user(1, "2".into());

        assert_eq!(result, "User 1 2");

        spy.verify();
    }
}
