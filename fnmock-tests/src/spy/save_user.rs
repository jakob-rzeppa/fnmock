//! A second spied function, `fn save_user(id: String)`, so a sequence has calls of two
//! different functions to order.
//!
//! Trimmed to what the cross-function tests need: one predicate per parameter, no `expectf`
//! and no global call count. Everything else works exactly as in [`super::save_user`].

/// The spied function.
pub fn save_user(id: String) {
    #[cfg(test)]
    save_user_spy().record_call(&(&id,));

    let _ = id;
}

/// The accessor a test reaches the spy through.
pub fn save_user_spy() -> save_user_spy_module::SaveUserSpyInterface {
    save_user_spy_module::SaveUserSpyInterface::new()
}

/// Everything generated for `save_user`, in the three parts a spy consists of: how a call's
/// arguments are matched, where the expectations live, and what the test may call.
pub mod save_user_spy_module {
    use std::{cell::RefCell, fmt::Display, rc::Rc};

    use fnmock::{
        Predicate, call_range::CallRange, expectation_handle::ExpectationHandle, matcher::Matcher,
        spy_store::SpyStore,
    };

    // ---- Matching -----------------------------------------------------------------------
    // One variant per way of expressing an expectation: a predicate per parameter, or a
    // single function over all of them.

    type SaveUserParams<'a> = (&'a String,);

    #[derive(Clone)]
    pub enum SaveUserMatcher {
        Predicates {
            id: Rc<dyn Predicate<String>>,
        },
        Function {
            function: Rc<dyn Fn(&String) -> bool>,
        },
    }

    impl Matcher for SaveUserMatcher {
        type Params<'a> = SaveUserParams<'a>;

        fn matches(&self, params: &Self::Params<'_>) -> bool {
            match self {
                Self::Predicates { id } => id.eval(params.0),
                Self::Function { function } => function(params.0),
            }
        }
    }

    impl Display for SaveUserMatcher {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Predicates { id } => {
                    // See the matching comment in `get_user.rs`: swapping "var" for the real
                    // parameter name gets an expression-like `id == "a"` out of the crate's
                    // own `var == "a"` rendering.
                    write!(f, "{}", id.to_string().replacen("var", "id", 1))
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
        static SPY: RefCell<SpyStore<SaveUserMatcher>> = RefCell::new(SpyStore::new("save_user"));
    }

    // --- Internals -----------------------------------------------------------------------
    // The part the internals outside this module interact with.

    pub(super) fn internal_record_call(params: &SaveUserParams<'_>) {
        SPY.with_borrow_mut(|spy| {
            spy.record_call(params);
        })
    }

    // ---- Interface ----------------------------------------------------------------------
    // The only part a test touches.

    pub struct SaveUserSpyInterface {}

    impl SaveUserSpyInterface {
        pub fn new() -> Self {
            Self {}
        }

        /// Expect calls whose arguments satisfy one predicate per parameter.
        pub fn expect(
            &self,
            id: impl Predicate<String> + 'static,
        ) -> ExpectationHandle<SaveUserMatcher> {
            self.set_expectation(SaveUserMatcher::Predicates { id: Rc::new(id) })
        }

        /// Expect calls whose arguments satisfy `function`.
        pub fn expectf(
            &self,
            function: impl Fn(&String) -> bool + 'static,
        ) -> ExpectationHandle<SaveUserMatcher> {
            self.set_expectation(SaveUserMatcher::Function {
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
        pub fn record_call(&self, params: &(&String,)) {
            SPY.with_borrow_mut(|spy| spy.record_call(params));
        }

        /// Assert every expectation set on `save_user` is fulfilled.
        pub fn assert(&self) {
            SPY.with_borrow(|spy| spy.assert());
        }

        fn set_expectation(&self, matcher: SaveUserMatcher) -> ExpectationHandle<SaveUserMatcher> {
            ExpectationHandle::new(
                matcher,
                "save_user",
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
