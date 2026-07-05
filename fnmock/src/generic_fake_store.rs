use std::{ any::{ Any, TypeId }, collections::HashMap, rc::Rc };

/// A store for fake implementations of generic functions, keyed by the TypeIds of their generic parameters.
/// This allows faking generic functions with different type parameters without needing a separate static variable for each combination of types.
///
/// The Store needs to know the number of generic parameters at compile time, which is specified by the `GENERIC_COUNT` const generic parameter.
///
/// We store the fake implementations as `Rc<dyn Any>` so that we can downcast them to the correct function type when retrieving them.
/// This allows us to store different function types in the same store.
///
/// In the macros we generate, we use `Box<dyn Fn(...) -> ...>` for the function type, which is a trait object that can be stored in the `Any` type.
/// This leads to the closures being stored as Rc<Box<dyn Fn(...) -> ...>> in the store. This is a workaround for the fact that we cannot store `Rc<dyn Fn(...) -> ...>` directly in the store, because `dyn Fn(...) -> ...` is not `Sized`, and therefore cannot be used as a parameter on its own.
pub struct GenericFakeStore<const GENERIC_COUNT: usize> {
    /// A name for the fake store, used in error messages to make it clear which function's fake store is being referred to.
    name: &'static str,
    /// Keyed by generic type ids; value is erased to `dyn Any` and downcast when retrieved.
    /// We use Rc to allow cloning the function pointer for multiple calls.
    impls: HashMap<[TypeId; GENERIC_COUNT], Rc<dyn Any>>,
}

impl<const GENERIC_COUNT: usize> GenericFakeStore<GENERIC_COUNT> {
    pub fn new(name: &'static str) -> Self {
        Self { name, impls: HashMap::new() }
    }

    /// Set a fake implementation for a specific combination of generic types.
    ///
    /// The `Function` type parameter should be a boxed closure that matches the signature of the faked function for the given combination of generic types.
    /// For example, if the function being faked is `fn foo<T, U>(x: T, y: U) -> String`, and you want to set a
    /// fake implementation for `T = i32` and `U = String`, then the `Function` type parameter should be `Box<dyn Fn(i32, String) -> String>`.
    ///
    /// You **NEED** to specify the type of the closure in the generic parameter of the `setup_for` function, otherwise the compiler might infer the wrong type and you will get a runtime panic when trying to retrieve the fake implementation.
    pub fn setup_for<WrappedClosure: 'static>(
        &mut self,
        generic_types: [TypeId; GENERIC_COUNT],
        f: WrappedClosure
    ) {
        self.impls.insert(generic_types, Rc::new(f));
    }

    /// Clear all fake implementations.
    pub fn clear_all(&mut self) {
        self.impls.clear();
    }

    /// Clear the fake implementation for a specific combination of generic types.
    pub fn clear_for(&mut self, generic_types: [TypeId; GENERIC_COUNT]) {
        self.impls.remove(&generic_types);
    }

    /// Check if a fake implementation is set for a specific combination of generic types.
    pub fn is_set_for(&self, generic_types: [TypeId; GENERIC_COUNT]) -> bool {
        self.impls.contains_key(&generic_types)
    }

    /// Get the fake implementation for a specific combination of generic types.
    ///
    /// The `Function` type parameter should be a boxed closure that matches the signature of the faked function for the given combination of generic types.
    /// It needs to match the generic that was passed to `setup_for` for the same combination of generic types exactly.
    ///
    /// Panics if no implementation is set for the given types or if the stored implementation cannot be downcast to the expected function type.
    pub fn get_for<WrappedClosure: 'static>(
        &self,
        generic_types: [TypeId; GENERIC_COUNT]
    ) -> Rc<WrappedClosure> {
        self.impls
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
            .downcast::<WrappedClosure>()
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
        let mut store = GenericFakeStore::<2>::new("test_fn");

        assert!(!store.is_set_for([TypeId::of::<i32>(), TypeId::of::<String>()]));
        assert!(!store.is_set_for([TypeId::of::<u32>(), TypeId::of::<String>()]));

        store.setup_for::<Box<dyn Fn(i32, String) -> String>>(
            [TypeId::of::<i32>(), TypeId::of::<String>()],
            Box::new(|a: i32, b: String| format!("Fake for i32, String: {} {}", a, b))
        );
        store.setup_for::<Box<dyn Fn(u32, String) -> String>>(
            [TypeId::of::<u32>(), TypeId::of::<String>()],
            Box::new(|a: u32, b: String| format!("Fake for u32, String: {} {}", a, b))
        );

        assert!(store.is_set_for([TypeId::of::<i32>(), TypeId::of::<String>()]));
        assert!(store.is_set_for([TypeId::of::<u32>(), TypeId::of::<String>()]));

        let f1 = store.get_for::<Box<dyn Fn(i32, String) -> String>>([
            TypeId::of::<i32>(),
            TypeId::of::<String>(),
        ]);
        let f2 = store.get_for::<Box<dyn Fn(u32, String) -> String>>([
            TypeId::of::<u32>(),
            TypeId::of::<String>(),
        ]);

        println!("Calling f1:");
        println!("{}", f1(42, "Alice".into()));
        println!("Calling f2:");
        println!("{}", f2(42, "Bob".into()));

        store.clear_all();

        assert!(!store.is_set_for([TypeId::of::<i32>(), TypeId::of::<String>()]));
        assert!(!store.is_set_for([TypeId::of::<u32>(), TypeId::of::<String>()]));
    }
}
