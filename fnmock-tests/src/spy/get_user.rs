//! One spied function, `fn get_user(id: String, uuid: &str) -> String`, and the code a macro
//! would generate for it.

/// The spied function. Everything but the `#[cfg(test)]` line is what the user wrote.
pub fn get_user(mut id: String, uuid: &str) -> String {
    #[cfg(test)]
    get_user_spy().record_call(&(&id, uuid));

    id.push_str(uuid);
    id
}

/// The accessor a test reaches the spy through.
pub fn get_user_spy() -> get_user_spy_module::GetUserSpyInterface {
    get_user_spy_module::GetUserSpyInterface::new()
}

/// Everything generated for `get_user`, in the three parts a spy consists of: how a call's
/// arguments are matched, where the expectations live, and what the test may call.
pub mod get_user_spy_module {
    use std::{cell::RefCell, fmt::Display, rc::Rc};

    use fnmock::{
        Predicate, call_range::CallRange, expectation_handle::ExpectationHandle, matcher::Matcher,
        spy_store::SpyStore,
    };

    // ---- Matching -----------------------------------------------------------------------
    // One variant per way of expressing an expectation: a predicate per parameter, or a
    // single function over all of them.

    type GetUserParams<'a> = (&'a String, &'a str);

    #[derive(Clone)]
    pub enum GetUserMatcher {
        Predicates {
            id: Rc<dyn Predicate<String>>,
            uuid: Rc<dyn Predicate<str>>,
        },
        Function {
            function: Rc<dyn Fn(&String, &str) -> bool>,
        },
    }

    impl Matcher for GetUserMatcher {
        type Params<'a> = GetUserParams<'a>;

        fn matches(&self, params: &Self::Params<'_>) -> bool {
            match self {
                Self::Predicates { id, uuid } => id.eval(params.0) && uuid.eval(params.1),
                Self::Function { function } => function(params.0, params.1),
            }
        }
    }

    impl Display for GetUserMatcher {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Predicates { id, uuid } => {
                    // `predicates` always names the compared value "var" (e.g. `var == "a"`);
                    // swapping in the real parameter name gets us an expression-like
                    // `id == "a"` for the common comparison predicates, and is a harmless no-op
                    // for predicates (like `always()`) that don't mention "var" at all.
                    write!(
                        f,
                        "{} && {}",
                        id.to_string().replacen("var", "id", 1),
                        uuid.to_string().replacen("var", "uuid", 1)
                    )
                }
                Self::Function { .. } => {
                    write!(f, "a function predicate")
                }
            }
        }
    }

    // ---- Storage ------------------------------------------------------------------------
    // One store per function, per thread, so tests running in parallel cannot see each
    // other's expectations.

    thread_local! {
        static SPY: RefCell<SpyStore<GetUserMatcher>> = RefCell::new(SpyStore::new("get_user"));
    }

    // --- Internals -----------------------------------------------------------------------
    // The part the internals outside this module interact with.

    pub(super) fn internal_record_call(params: &GetUserParams<'_>) {
        SPY.with_borrow_mut(|spy| {
            spy.record_call(params);
        })
    }

    // ---- Interface ----------------------------------------------------------------------
    // The only part a test touches.

    pub struct GetUserSpyInterface {}

    impl GetUserSpyInterface {
        pub fn new() -> Self {
            Self {}
        }

        /// Expect calls whose arguments satisfy one predicate per parameter.
        pub fn expect(
            &self,
            id: impl Predicate<String> + 'static,
            uuid: impl Predicate<str> + 'static,
        ) -> ExpectationHandle<GetUserMatcher> {
            self.set_expectation(GetUserMatcher::Predicates {
                id: Rc::new(id),
                uuid: Rc::new(uuid),
            })
        }

        /// Expect calls whose arguments satisfy `function`.
        pub fn expectf(
            &self,
            function: impl Fn(&String, &str) -> bool + 'static,
        ) -> ExpectationHandle<GetUserMatcher> {
            self.set_expectation(GetUserMatcher::Function {
                function: Rc::new(function),
            })
        }

        /// Expect this many calls, whatever their arguments.
        pub fn expect_times(&self, call_range: impl Into<CallRange>) {
            SPY.with_borrow_mut(|spy| spy.set_total_call_range(call_range.into()));
        }

        /// Expect exactly one call, whatever its arguments.
        pub fn expect_once(&self) {
            self.expect_times(1);
        }

        /// Expect the function not to be called at all.
        pub fn expect_never(&self) {
            self.expect_times(0);
        }

        /// Record a call. Called by the spied function, not by the test.
        pub fn record_call(&self, params: &(&String, &str)) {
            SPY.with_borrow_mut(|spy| spy.record_call(params));
        }

        /// Assert every expectation set on `get_user` is fulfilled.
        pub fn assert(&self) {
            SPY.with_borrow(|spy| spy.assert());
        }

        fn set_expectation(&self, matcher: GetUserMatcher) -> ExpectationHandle<GetUserMatcher> {
            ExpectationHandle::new(
                matcher,
                "get_user",
                |expectation| {
                    SPY.with_borrow_mut(|spy| {
                        spy.add_expectation(expectation);
                    })
                },
                |sequences| {
                    SPY.with_borrow_mut(|spy| {
                        spy.add_sequences(sequences);
                    })
                },
            )
        }
    }
}
