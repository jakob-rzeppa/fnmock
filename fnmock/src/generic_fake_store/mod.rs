//! Storage backing the fake of a generic function, keyed by the generic arguments it was called with.

use std::{any::Any, collections::HashMap, rc::Rc};

use crate::generic_fake_store::key::GenericKeyPart;

pub mod key;

/// A store for fake implementations of generic functions, keyed by `GenericKeyPart`s built from their generic parameters.
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
    /// Keyed by generic key parts (TypeId for type parameters, value for const parameters); value is erased to `dyn Any` and downcast when retrieved.
    /// We use Rc to allow cloning the function pointer for multiple calls.
    impls: HashMap<[GenericKeyPart; GENERIC_COUNT], Rc<dyn Any>>,
}

impl<const GENERIC_COUNT: usize> GenericFakeStore<GENERIC_COUNT> {
    /// Create an empty store for the function named `name`.
    ///
    /// The name is only used to identify the function in panic messages.
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            impls: HashMap::new(),
        }
    }

    /// Set a fake implementation for a specific combination of generic types.
    ///
    /// The `WrappedClosure` type parameter should be a boxed closure that matches the signature of the faked function for the given combination of generic types.
    /// For example, if the function being faked is `fn foo<T, U>(x: T, y: U) -> String`, and you want to set a
    /// fake implementation for `T = i32` and `U = String`, then the `WrappedClosure` type parameter should be `Box<dyn Fn(i32, String) -> String>`.
    ///
    /// You **NEED** to specify the type of the closure in the generic parameter of the `setup_for` function, otherwise the compiler might infer the wrong type and you will get a runtime panic when trying to retrieve the fake implementation.
    pub fn setup_for<WrappedClosure: 'static>(
        &mut self,
        generic_keys: [GenericKeyPart; GENERIC_COUNT],
        f: WrappedClosure,
    ) {
        self.impls.insert(generic_keys, Rc::new(f));
    }

    /// Clear the fake implementation for a specific combination of generic types.
    ///
    /// Fakes registered for other combinations are left untouched.
    pub fn clear_for(&mut self, generic_keys: [GenericKeyPart; GENERIC_COUNT]) {
        self.impls.remove(&generic_keys);
    }

    /// Check if a fake implementation is set for a specific combination of generic types.
    ///
    /// The inline call the macro injects checks this before calling [`GenericFakeStore::get_for`],
    /// so that a call with generic arguments that were never faked falls through to the real body.
    pub fn is_set_for(&self, generic_keys: [GenericKeyPart; GENERIC_COUNT]) -> bool {
        self.impls.contains_key(&generic_keys)
    }

    /// Get the fake implementation for a specific combination of generic types.
    ///
    /// The `WrappedClosure` type parameter should be a boxed closure that matches the signature of the faked function for the given combination of generic types.
    /// It needs to match the generic that was passed to `setup_for` for the same combination of generic types exactly.
    ///
    /// # Panics
    ///
    /// Panics if no implementation is set for the given types or if the stored implementation cannot be downcast to the expected function type.
    pub fn get_for<WrappedClosure: 'static>(
        &self,
        generic_keys: [GenericKeyPart; GENERIC_COUNT],
    ) -> Rc<WrappedClosure> {
        self.impls
            .get(&generic_keys)
            .cloned()
            .unwrap_or_else(|| {
                // When using the macro API, the macro ensures that get is only called when is_set_for is true, so this should never happen if the API is used correctly.
                panic!(
                    "Generic fake {} for {:#?} should only be called with initialized types, since is_set_for is checked before calling. This should never happen if the API is used correctly.",
                    self.name,
                    generic_keys
                )
            })
            .downcast::<WrappedClosure>()
            .unwrap_or_else(|_| {
                // When using the macro API, the macro ensures that the type of get_for and setup_for match, so this should never happen if the API is used correctly.
                panic!(
                    "Downcast of generic fake {} for {:#?} failed. This should never happen if the API is used correctly. Expected function type does not match the type of the provided implementation.",
                    self.name,
                    generic_keys
                );
            })
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use crate::generic_fake_store::key::ConstValue;

    use super::*;

    #[test]
    fn test_generic_fake_store() {
        let mut store = GenericFakeStore::<2>::new("test_fn");

        let i32_string_key = [
            GenericKeyPart::Type(TypeId::of::<i32>()),
            GenericKeyPart::Type(TypeId::of::<String>()),
        ];
        let u32_string_key = [
            GenericKeyPart::Type(TypeId::of::<u32>()),
            GenericKeyPart::Type(TypeId::of::<String>()),
        ];

        assert!(!store.is_set_for(i32_string_key.clone()));
        assert!(!store.is_set_for(u32_string_key.clone()));

        store.setup_for::<Box<dyn Fn(i32, String) -> String>>(
            i32_string_key.clone(),
            Box::new(|a: i32, b: String| format!("Fake for i32, String: {} {}", a, b)),
        );
        store.setup_for::<Box<dyn Fn(u32, String) -> String>>(
            u32_string_key.clone(),
            Box::new(|a: u32, b: String| format!("Fake for u32, String: {} {}", a, b)),
        );

        assert!(store.is_set_for(i32_string_key.clone()));
        assert!(store.is_set_for(u32_string_key.clone()));

        let f1 = store.get_for::<Box<dyn Fn(i32, String) -> String>>(i32_string_key.clone());
        let f2 = store.get_for::<Box<dyn Fn(u32, String) -> String>>(u32_string_key.clone());

        println!("Calling f1:");
        println!("{}", f1(42, "Alice".into()));
        println!("Calling f2:");
        println!("{}", f2(42, "Bob".into()));

        store.clear_for(i32_string_key.clone());
        store.clear_for(u32_string_key.clone());

        assert!(!store.is_set_for(i32_string_key));
        assert!(!store.is_set_for(u32_string_key));
    }

    #[test]
    fn test_const_generic_values_do_not_collide() {
        // Simulates a store for `fn foo<const C: usize>()`: two different
        // values of C must be independent keys, not just the same TypeId.
        let mut store = GenericFakeStore::<1>::new("test_fn");

        store.setup_for::<Box<dyn Fn() -> &'static str>>(
            [GenericKeyPart::Const(ConstValue::new(5usize))],
            Box::new(|| "fake for C=5"),
        );

        assert!(store.is_set_for([GenericKeyPart::Const(ConstValue::new(5usize))]));
        assert!(!store.is_set_for([GenericKeyPart::Const(ConstValue::new(7usize))]));
    }

    #[test]
    fn test_const_value_supports_arbitrary_hash_eq_types() {
        // ConstValue must not be limited to a hardcoded set of primitive types: any
        // Hash + Eq + 'static value should work as a const generic key part.
        #[derive(Hash, PartialEq, Eq)]
        struct CustomKey(u8, bool);

        let mut store = GenericFakeStore::<1>::new("test_fn");
        store.setup_for::<Box<dyn Fn() -> &'static str>>(
            [GenericKeyPart::Const(ConstValue::new(CustomKey(1, true)))],
            Box::new(|| "fake"),
        );

        assert!(store.is_set_for([GenericKeyPart::Const(ConstValue::new(CustomKey(1, true)))]));
        assert!(!store.is_set_for([GenericKeyPart::Const(ConstValue::new(CustomKey(2, true)))]));
    }
}
