use std::{ any::Any, rc::Rc };

/// A store for a single fake function implementation.
///
/// This is used for non-generic functions, or for generic functions where we ignore the generic parameters and therefore only have one implementation.
pub struct FakeStore<Function: Any + 'static> {
    /// Function name for error messages.
    name: &'static str,
    /// Optional custom implementation function. Wrapped in Rc to allow cloning the function pointer for multiple calls.
    implementation: Option<Rc<Function>>,
}

impl<Function: Any + 'static> FakeStore<Function> {
    pub fn new(function_name: &'static str) -> Self {
        Self {
            name: function_name,
            implementation: None,
        }
    }

    /// Set a fake implementation for the function.
    ///
    /// The `Function` type parameter should be a function pointer that matches the signature of the function being faked.
    pub fn setup(&mut self, new_f: Function) {
        self.implementation = Some(Rc::new(new_f));
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
    pub fn get(&self) -> Rc<Function> {
        self.implementation.clone().unwrap_or_else(|| {
            // When using the macro API, the macro ensures that get is only called when is_set is true, so this should never happen if the API is used correctly.
            unreachable!(
                "Fake {} should only be called when initialized, since is_set is checked before calling. This should never happen if the API is used correctly.",
                self.name
            );
        })
    }
}
