/// A store for a single fake function implementation.
///
/// This is used for non-generic functions, or for generic functions where we ignore the generic parameters and therefore only have one implementation.
///
/// # Generics
///
/// - `WrappedClosure`: A closure trait for the faked function, wrapped in a `Rc` to allow for cloning. The closure trait should match the signature of the function being faked. For example, for a function `fn foo(x: i32) -> i32`, the `WrappedClosure` type parameter would be `Rc<dyn Fn(i32) -> i32>`. This is handled by the proc-macro, so you don't need to worry about it when using the macro API.
pub struct FakeStore<WrappedClosure: Clone> {
    /// Function name for error messages.
    name: &'static str,
    /// Optional custom implementation.
    implementation: Option<WrappedClosure>,
}

impl<WrappedClosure: Clone> FakeStore<WrappedClosure> {
    pub fn new(function_name: &'static str) -> Self {
        Self {
            name: function_name,
            implementation: None,
        }
    }

    /// Set a fake implementation for the function.
    ///
    /// The `WrappedClosure` type parameter should be a `Rc`-wrapped closure trait that matches the signature of the function being faked.
    pub fn setup(&mut self, new_f: WrappedClosure) {
        self.implementation = Some(new_f);
    }

    /// Clear the fake implementation.
    pub fn clear(&mut self) {
        self.implementation = None;
    }

    /// Check if a fake implementation is set.
    pub fn is_set(&self) -> bool {
        self.implementation.is_some()
    }

    /// Get the fake implementation.
    ///
    /// Panics if no implementation is set. When using the macro API, the macro ensures that get is only called when is_set is true, so this should never happen if the API is used correctly.
    pub fn get(&self) -> WrappedClosure {
        self.implementation.clone().unwrap_or_else(|| {
            // When using the macro API, the macro ensures that get is only called when is_set is true, so this should never happen if the API is used correctly.
            unreachable!(
                "Fake {} should only be called when initialized, since is_set is checked before calling. This should never happen if the API is used correctly.",
                self.name
            );
        })
    }
}
