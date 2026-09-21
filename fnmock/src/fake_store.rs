//! Storage backing the fake of a non-generic function.

/// A store for a single fake function implementation.
///
/// This is used for non-generic functions, or for generic functions where we ignore the generic parameters and therefore only have one implementation.
///
/// # Generics
///
/// - `WrappedClosure`: A closure trait for the faked function, wrapped in a `Rc` to allow for cloning. The closure trait should match the signature of the function being faked. For example, for a function `fn foo(x: i32) -> i32`, the `WrappedClosure` type parameter would be `Rc<dyn Fn(i32) -> i32>`. This is handled by the proc-macro, so you don't need to worry about it when using the macro API.
pub struct FakeStore<WrappedClosure: Clone> {
    /// Function name for error messages.
    _name: &'static str,
    /// Optional custom implementation.
    implementation: Option<WrappedClosure>,
}

impl<WrappedClosure: Clone> FakeStore<WrappedClosure> {
    /// Create an empty store for the function named `function_name`.
    ///
    /// The name is only used to identify the function in panic messages.
    pub fn new(function_name: &'static str) -> Self {
        Self {
            _name: function_name,
            implementation: None,
        }
    }

    /// Set a fake implementation for the function, replacing any previously set one.
    ///
    /// The `WrappedClosure` type parameter should be a `Rc`-wrapped closure trait that matches the signature of the function being faked.
    pub fn setup(&mut self, new_f: WrappedClosure) {
        self.implementation = Some(new_f);
    }

    /// Clear the fake implementation, so calls run the real function body again.
    pub fn clear(&mut self) {
        self.implementation = None;
    }

    /// Check if a fake implementation is set.
    ///
    /// The inline call the macro injects checks this before calling [`FakeStore::get`], so that a
    /// function without a fake falls through to its real body.
    pub fn is_set(&self) -> bool {
        self.implementation.is_some()
    }

    /// Get the fake implementation.
    pub fn get(&self) -> Option<WrappedClosure> {
        self.implementation.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn new_store_has_no_implementation_set() {
        let store: FakeStore<Rc<dyn Fn() -> i32>> = FakeStore::new("test_fn");

        assert!(!store.is_set());
    }

    #[test]
    fn setup_marks_implementation_as_set() {
        let mut store: FakeStore<Rc<dyn Fn() -> i32>> = FakeStore::new("test_fn");

        store.setup(Rc::new(|| 42));

        assert!(store.is_set());
    }

    #[test]
    fn get_returns_the_implementation_passed_to_setup() {
        let mut store: FakeStore<Rc<dyn Fn(i32) -> i32>> = FakeStore::new("test_fn");

        store.setup(Rc::new(|x: i32| x + 1));

        let f = store.get();
        assert!(f.is_some());
        assert_eq!(f.unwrap()(41), 42);
    }

    #[test]
    fn setup_overwrites_previous_implementation() {
        let mut store: FakeStore<Rc<dyn Fn() -> i32>> = FakeStore::new("test_fn");

        store.setup(Rc::new(|| 1));
        store.setup(Rc::new(|| 2));

        let f = store.get();
        assert!(f.is_some());
        assert_eq!(f.unwrap()(), 2);
    }

    #[test]
    fn clear_removes_the_implementation() {
        let mut store: FakeStore<Rc<dyn Fn() -> i32>> = FakeStore::new("test_fn");
        store.setup(Rc::new(|| 42));

        store.clear();

        assert!(!store.is_set());
    }

    #[test]
    fn get_returns_none_when_no_implementation_is_set() {
        let store: FakeStore<Rc<dyn Fn() -> i32>> = FakeStore::new("test_fn");

        assert!(store.get().is_none());
    }
}
