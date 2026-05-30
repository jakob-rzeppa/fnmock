use std::{ any::{ Any, TypeId }, cell::RefCell, collections::HashMap, rc::Rc };

/// A store for fake implementations of generic functions, keyed by the TypeIds of their generic parameters.
/// This allows faking generic functions with different type parameters without needing a separate static variable for each combination of types.
///
///
/// # Do not use this struct directly!
///
/// You should never interact with this struct directly. Instead, use the `#[fake_function]` macro which generates a wrapper
/// around this store for each generic function, providing a convenient API for setting and getting fake implementations based on the generic type parameters.
///
/// When using the macro `#[fake_function]`, the generated proxy module will ensure that you can only set and get fake implementations
/// for the specific generic type combinations that are supported by the macro, and it will handle the TypeId management for you.
/// The macro will also ensure that you can only get an implementation if it has been set, so you don't have to worry about
/// handling missing implementations when using the generated API.
pub struct GenericFakeStore<const GENERIC_COUNT: usize> {
    /// A name for the fake store, used in error messages to make it clear which function's fake store is being referred to.
    name: &'static str,
    /// Keyed by generic type ids; value is erased to `dyn Any` and downcast when retrieved.
    /// We use Rc to allow cloning the function pointer for multiple calls.
    impls: RefCell<HashMap<[TypeId; GENERIC_COUNT], Rc<dyn Any>>>,
}

impl<const GENERIC_COUNT: usize> GenericFakeStore<GENERIC_COUNT> {
    pub fn new(name: &'static str) -> Self {
        Self { name, impls: RefCell::new(HashMap::new()) }
    }

    /// Set a fake implementation for a specific combination of generic types.
    ///
    /// The `Function` type parameter should be a function pointer that matches the signature of the function being faked,
    /// with the generic parameters replaced by concrete types. For example, if faking a function `fn foo<T>(x: T) -> String`,
    /// the `Function` type parameter for setting a fake implementation for `T = i32` should be `fn(i32) -> String`.
    ///
    /// The `generic_types` parameter is an array of `TypeId` that specifies the concrete types for the generic parameters.
    /// The order of types in the array should match the order of generic parameters in the function signature.
    /// For example, for a function `fn foo<T, U>(x: T, y: U) -> String`, if setting a fake implementation for `T = i32` and `U = String`,
    /// the `generic_types` parameter should be `[TypeId::of::<i32>(), TypeId::of::<String>()]`.
    pub fn setup_for<Function: Any + 'static>(
        &self,
        generic_types: [TypeId; GENERIC_COUNT],
        f: Function
    ) {
        self.impls.borrow_mut().insert(generic_types, Rc::new(f));
    }

    /// Clear all fake implementations.
    pub fn clear(&self) {
        self.impls.borrow_mut().clear();
    }

    /// Clear the fake implementation for a specific combination of generic types.
    pub fn clear_for(&self, generic_types: [TypeId; GENERIC_COUNT]) {
        self.impls.borrow_mut().remove(&generic_types);
    }

    /// Check if a fake implementation is set for a specific combination of generic types.
    pub fn is_set_for(&self, generic_types: [TypeId; GENERIC_COUNT]) -> bool {
        self.impls.borrow().contains_key(&generic_types)
    }

    /// Get the fake implementation for a specific combination of generic types.
    ///
    /// Panics if no implementation is set for the given types or if the stored implementation cannot be downcast to the expected function type.
    ///
    /// The `Function` type parameter should match the type of the function pointer that was set for the given generic types.
    /// For example, if a fake implementation for `T = i32` and `U = String` was set using a function pointer of type `fn(i32, String) -> String`,
    /// then the `Function` type parameter for getting that implementation should also be `fn(i32, String) -> String`.
    pub fn get_for<Function: Any + 'static>(
        &self,
        generic_types: [TypeId; GENERIC_COUNT]
    ) -> Rc<Function> {
        self.impls
            .borrow()
            .get(&generic_types)
            .cloned()
            .unwrap_or_else(|| {
                // When using the macro API, the macro ensures that get is only called when is_set_for is true, so this should never happen if the API is used correctly.
                unreachable!(
                    "Generic fake {} for {:#?} should only be called with initialized types, since is_set_for is checked before calling. This should never happen if the API is used correctly.",
                    self.name,
                    generic_types
                )
            })
            .downcast::<Function>()
            .unwrap_or_else(|_| {
                // When using the macro API, the macro ensures that the type of get_for and setup_for match, so this should never happen if the API is used correctly.
                unreachable!(
                    "Downcast of generic fake {} for {:#?} failed. This should never happen if the API is used correctly. Expected function type does not match the type of the provided implementation.",
                    self.name,
                    generic_types
                );
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_fake_store() {
        let store = GenericFakeStore::<2>::new("test_fn");

        assert!(!store.is_set_for([TypeId::of::<i32>(), TypeId::of::<String>()]));
        assert!(!store.is_set_for([TypeId::of::<u32>(), TypeId::of::<String>()]));

        store.setup_for::<fn(i32, String) -> String>(
            [TypeId::of::<i32>(), TypeId::of::<String>()],
            |a: i32, b: String| "Fake for i32, String".into()
        );
        store.setup_for::<fn(u32, String) -> String>(
            [TypeId::of::<u32>(), TypeId::of::<String>()],
            |a: u32, b: String| "Fake for u32, String".into()
        );

        assert!(store.is_set_for([TypeId::of::<i32>(), TypeId::of::<String>()]));
        assert!(store.is_set_for([TypeId::of::<u32>(), TypeId::of::<String>()]));

        let f1 = store.get_for::<fn(i32, String) -> String>([
            TypeId::of::<i32>(),
            TypeId::of::<String>(),
        ]);
        let f2 = store.get_for::<fn(u32, String) -> String>([
            TypeId::of::<u32>(),
            TypeId::of::<String>(),
        ]);

        println!("Calling f1:");
        println!("{}", f1(42, "Alice".into()));
        println!("Calling f2:");
        println!("{}", f2(42, "Bob".into()));

        store.clear();

        assert!(!store.is_set_for([TypeId::of::<i32>(), TypeId::of::<String>()]));
        assert!(!store.is_set_for([TypeId::of::<u32>(), TypeId::of::<String>()]));
    }
}
