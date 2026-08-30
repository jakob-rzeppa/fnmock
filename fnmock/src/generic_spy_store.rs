//! Storage backing the spy of a generic function, keyed by the generic arguments it was called
//! with.
//!
//! This is a fnmock internal. You should not interact with it directly.

use std::{any::Any, collections::HashMap};

use crate::{generic_fake_store::key::GenericKeyPart, matcher::Matcher, spy_store::SpyStore};

/// Dyn-safe view of a [`SpyStore<M>`], for holding the stores of different instantiations of one
/// generic function alongside each other.
///
/// The matcher of a generic function is generic too — `Matcher<String>` and `Matcher<i32>` are
/// unrelated types — so the stores can only be kept in one collection type-erased.
pub trait DynSpyStore: Any {
    /// See [`SpyStore::name`].
    fn name(&self) -> &str;
    /// See [`SpyStore::assert_failures`].
    fn check_for_failures(&self) -> Vec<String>;
    /// Borrow this store as [`Any`], to downcast to a concrete `SpyStore<M>`.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<M: Matcher> DynSpyStore for SpyStore<M> {
    fn name(&self) -> &str {
        SpyStore::name(self)
    }

    fn check_for_failures(&self) -> Vec<String> {
        SpyStore::check_for_failures(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// The spy of a generic function: one independent [`SpyStore`] per combination of generic
/// arguments, keyed by [`GenericKeyPart`]s.
///
/// The number of generic parameters is fixed at compile time by `GENERIC_COUNT`.
///
/// Expectations set on one instantiation never see calls made with other generic arguments, and
/// `assert` is likewise per instantiation — [`GenericSpyStore::assert_all`] is the sweep that
/// makes a forgotten instantiation impossible to miss.
pub struct GenericSpyStore<const GENERIC_COUNT: usize> {
    /// The spied function's name, without any generic arguments, for the header of a failure
    /// covering several instantiations at once.
    name: &'static str,
    /// One store per combination of generic arguments, created on first use. An instantiation
    /// that was never expected on and never called simply has no entry.
    ///
    /// This behavior is consistent with expectations and sequences in general, since they always ignore
    /// unexpected calls.
    stores: HashMap<[GenericKeyPart; GENERIC_COUNT], Box<dyn DynSpyStore>>,
}

impl<const GENERIC_COUNT: usize> GenericSpyStore<GENERIC_COUNT> {
    /// Create a store with no instantiations recorded yet.
    ///
    /// `name` is the plain function name, e.g. `identity`. Each instantiation names itself in
    /// full when it is created; see [`GenericSpyStore::with_store_mut`].
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            stores: HashMap::new(),
        }
    }

    /// Run `f` against the store for one combination of generic arguments, creating it if this
    /// is the first time that combination is seen.
    ///
    /// `name` is only called when the store has to be created, so recording a call on an
    /// already-known instantiation does not build the label.
    ///
    /// # Panics
    ///
    /// Panics if a store already registered under `generic_keys` was created with a different
    /// matcher type. The generated code derives both the key and `M` from the same generic
    /// arguments, so this would be a bug in fnmock rather than a user error.
    pub fn with_store_mut<M: Matcher, R>(
        &mut self,
        generic_keys: [GenericKeyPart; GENERIC_COUNT],
        name: impl FnOnce() -> String,
        f: impl FnOnce(&mut SpyStore<M>) -> R,
    ) -> R {
        let store = self
            .stores
            .entry(generic_keys)
            .or_insert_with(|| Box::new(SpyStore::<M>::new(name())));

        let store = store.as_any_mut().downcast_mut::<SpyStore<M>>().unwrap_or_else(|| {
            panic!(
                "Generic spy '{}' holds a store whose matcher type does not match the generic arguments it is keyed by. This is a bug in fnmock; please report it.",
                self.name
            )
        });

        f(store)
    }

    /// Assert that every expectation set on one combination of generic arguments is fulfilled.
    ///
    /// An instantiation that was never expected on has nothing to check, so this passes.
    ///
    /// # Panics
    ///
    /// Panics if any expectation of that instantiation is not fulfilled.
    pub fn assert_for(&self, generic_keys: &[GenericKeyPart; GENERIC_COUNT]) {
        let Some(store) = self.stores.get(generic_keys) else {
            return;
        };

        let failures = store.check_for_failures();

        assert!(
            failures.is_empty(),
            "Expectation(s) of the spied function '{}' failed:\n{}",
            store.name(),
            failures.join("\n")
        );
    }

    /// Assert that every expectation set on *any* combination of generic arguments is fulfilled,
    /// reporting the failures of all of them together.
    ///
    /// Each failure is prefixed with the instantiation it came from, since the same expectation
    /// can be set on several and the messages would otherwise be indistinguishable.
    ///
    /// # Panics
    ///
    /// Panics if any expectation of any instantiation is not fulfilled.
    pub fn assert_all(&self) {
        let mut failures: Vec<String> = Vec::new();

        for store in self.stores.values() {
            for failure in store.check_for_failures() {
                failures.push(format!("{}: {}", store.name(), failure));
            }
        }

        // `HashMap` iteration order varies between runs, so sort to keep the message stable.
        failures.sort();

        assert!(
            failures.is_empty(),
            "Expectation(s) of the spied function '{}' failed:\n{}",
            self.name,
            failures.join("\n")
        );
    }
}

#[cfg(test)]
mod tests {
    use std::{any::TypeId, fmt::Display, marker::PhantomData};

    use crate::{expectation::Expectation, generic_fake_store::key::ConstValue};

    use super::*;

    /// Stands in for the matcher the macro generates for `fn f<T>(a: T)`: generic over `T`, and
    /// so a different type for every instantiation, which is the whole reason the stores have to
    /// be type-erased.
    struct TestMatcher<T> {
        accepts: bool,
        _phantom: PhantomData<fn() -> T>,
    }

    // Hand-written rather than derived: `#[derive(Clone)]` would add a `T: Clone` bound, which
    // `Matcher: Clone` would then demand of every spied generic function. The generated matcher
    // has to do the same.
    impl<T> Clone for TestMatcher<T> {
        fn clone(&self) -> Self {
            Self {
                accepts: self.accepts,
                _phantom: PhantomData,
            }
        }
    }

    impl<T> TestMatcher<T> {
        fn new(accepts: bool) -> Self {
            Self {
                accepts,
                _phantom: PhantomData,
            }
        }
    }

    impl<T> Display for TestMatcher<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "test matcher")
        }
    }

    impl<T: 'static> Matcher for TestMatcher<T> {
        type Params<'a> = (&'a T,);

        fn matches(&self, _params: &Self::Params<'_>) -> bool {
            self.accepts
        }
    }

    fn type_key<T: 'static>() -> [GenericKeyPart; 1] {
        [GenericKeyPart::Type(TypeId::of::<T>())]
    }

    #[test]
    fn test_each_combination_of_generic_arguments_gets_its_own_store() {
        let mut store = GenericSpyStore::<1>::new("f");

        store.with_store_mut::<TestMatcher<i32>, _>(
            type_key::<i32>(),
            || "f::<i32>".into(),
            |s| {
                s.add_expectation(Expectation::new(TestMatcher::new(true), "f::<i32>"));
                s.record_call(&(&1,));
            },
        );

        // A call on the i32 instantiation must not have reached the String one.
        store.with_store_mut::<TestMatcher<String>, _>(
            type_key::<String>(),
            || "f::<alloc::string::String>".into(),
            |s| {
                let expectation = Expectation::new(TestMatcher::new(true), "f::<String>");
                s.add_expectation(expectation);
                assert_eq!(s.check_for_failures().len(), 1);
            },
        );

        store.assert_for(&type_key::<i32>());
    }

    #[test]
    fn test_the_name_closure_only_runs_when_the_store_is_created() {
        let mut store = GenericSpyStore::<1>::new("f");
        let mut built = 0;

        for _ in 0..3 {
            store.with_store_mut::<TestMatcher<i32>, _>(
                type_key::<i32>(),
                || {
                    built += 1;
                    "f::<i32>".into()
                },
                |s| s.record_call(&(&1,)),
            );
        }

        assert_eq!(built, 1);
    }

    #[test]
    fn test_const_generic_values_are_separate_instantiations() {
        let mut store = GenericSpyStore::<1>::new("f");

        store.with_store_mut::<TestMatcher<i32>, _>(
            [GenericKeyPart::Const(ConstValue::new(5usize))],
            || "f::<5>".into(),
            |s| s.add_expectation(Expectation::new(TestMatcher::new(true), "f::<5>")),
        );

        // C = 7 was never touched, so it has nothing to check.
        store.assert_for(&[GenericKeyPart::Const(ConstValue::new(7usize))]);
    }

    #[test]
    fn test_assert_for_passes_for_an_instantiation_that_was_never_touched() {
        let store = GenericSpyStore::<1>::new("f");

        store.assert_for(&type_key::<i32>());
        store.assert_all();
    }

    #[test]
    fn test_assert_for_ignores_another_instantiations_failure() {
        let mut store = GenericSpyStore::<1>::new("f");

        store.with_store_mut::<TestMatcher<i32>, _>(
            type_key::<i32>(),
            || "f::<i32>".into(),
            |s| {
                s.add_expectation(Expectation::new(TestMatcher::new(true), "f::<i32>"));
            },
        );

        // The i32 instantiation is unfulfilled, but String is what we are asserting.
        store.with_store_mut::<TestMatcher<String>, _>(
            type_key::<String>(),
            || "f::<String>".into(),
            |s| s.record_call(&(&"a".to_string(),)),
        );

        store.assert_for(&type_key::<String>());
    }

    #[test]
    #[should_panic(expected = "f::<i32>")]
    fn test_assert_all_reports_a_failure_from_any_instantiation() {
        let mut store = GenericSpyStore::<1>::new("f");

        store.with_store_mut::<TestMatcher<i32>, _>(
            type_key::<i32>(),
            || "f::<i32>".into(),
            |s| {
                s.add_expectation(Expectation::new(TestMatcher::new(true), "f::<i32>"));
            },
        );

        store.assert_all();
    }
}
