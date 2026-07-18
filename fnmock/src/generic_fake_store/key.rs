use std::{
    any::{Any, TypeId},
    hash::{Hash, Hasher},
    rc::Rc,
};

/// Internal helper giving a type-erased value dynamic `Hash`/`Eq`, via `Any` downcasting.
/// This is what lets `ConstValue` accept any `T: Hash + Eq + 'static` without `GenericFakeStore`
/// (or the `#[fakeable]` macro) needing to enumerate every type that can be a const generic value.
trait DynHashEq {
    fn as_any(&self) -> &dyn Any;
    fn dyn_hash(&self, state: &mut dyn Hasher);
    fn dyn_eq(&self, other: &dyn DynHashEq) -> bool;
}

impl<T: Hash + Eq + 'static> DynHashEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        // Mix the concrete type into the hash too, so distinct types with an equal-looking
        // value (e.g. 5u8 vs 5u32) don't produce the same hash.
        TypeId::of::<T>().hash(&mut state);
        self.hash(&mut state);
    }

    fn dyn_eq(&self, other: &dyn DynHashEq) -> bool {
        other
            .as_any()
            .downcast_ref::<T>()
            .is_some_and(|other| self == other)
    }
}

/// The value of a const generic parameter, used as part of a `GenericFakeStore` key.
///
/// Type-erased via `Any` (through `DynHashEq`) so any `T: Hash + Eq + 'static` can be used,
/// without hardcoding the set of types stable Rust currently allows as const generic parameters.
#[derive(Clone)]
pub struct ConstValue(Rc<dyn DynHashEq>);

impl ConstValue {
    pub fn new<T: Hash + Eq + 'static>(value: T) -> Self {
        ConstValue(Rc::new(value))
    }
}

impl PartialEq for ConstValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.dyn_eq(&*other.0)
    }
}

impl Eq for ConstValue {}

impl Hash for ConstValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.dyn_hash(state);
    }
}

impl std::fmt::Debug for ConstValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ConstValue")
            .field(&self.0.as_any().type_id())
            .finish()
    }
}

/// One slot of a `GenericFakeStore` key: either the `TypeId` of a type parameter, or the value of a const parameter.
///
/// Const parameters must be keyed by their actual value, not just the `TypeId` of their type — otherwise every
/// value of e.g. `const C: usize` would collapse onto the single key `TypeId::of::<usize>()`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GenericKeyPart {
    Type(TypeId),
    Const(ConstValue),
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::hash::{DefaultHasher, Hash, Hasher};

    use super::*;

    fn hash_of<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn const_values_with_equal_value_and_type_are_equal() {
        assert_eq!(ConstValue::new(5u32), ConstValue::new(5u32));
    }

    #[test]
    fn const_values_with_different_value_are_not_equal() {
        assert_ne!(ConstValue::new(5u32), ConstValue::new(7u32));
    }

    #[test]
    fn const_values_with_same_value_but_different_type_are_not_equal() {
        // 5u8 and 5u32 must not collide just because they look equal as numbers.
        assert_ne!(ConstValue::new(5u8), ConstValue::new(5u32));
    }

    #[test]
    fn const_values_support_arbitrary_hash_eq_types() {
        #[derive(Hash, PartialEq, Eq)]
        struct CustomKey(u8, bool);

        assert_eq!(
            ConstValue::new(CustomKey(1, true)),
            ConstValue::new(CustomKey(1, true))
        );
        assert_ne!(
            ConstValue::new(CustomKey(1, true)),
            ConstValue::new(CustomKey(2, true))
        );
    }

    #[test]
    fn equal_const_values_have_equal_hashes() {
        assert_eq!(
            hash_of(&ConstValue::new(42usize)),
            hash_of(&ConstValue::new(42usize))
        );
    }

    #[test]
    fn const_values_with_same_value_but_different_type_have_different_hashes() {
        // The type is mixed into the hash so 5u8 and 5u32 don't share a bucket.
        assert_ne!(
            hash_of(&ConstValue::new(5u8)),
            hash_of(&ConstValue::new(5u32))
        );
    }

    #[test]
    fn cloned_const_value_is_equal_to_original() {
        let value = ConstValue::new(String::from("hello"));

        assert_eq!(value.clone(), value);
    }

    #[test]
    fn type_key_parts_with_same_type_id_are_equal() {
        assert_eq!(
            GenericKeyPart::Type(TypeId::of::<i32>()),
            GenericKeyPart::Type(TypeId::of::<i32>())
        );
    }

    #[test]
    fn type_key_parts_with_different_type_id_are_not_equal() {
        assert_ne!(
            GenericKeyPart::Type(TypeId::of::<i32>()),
            GenericKeyPart::Type(TypeId::of::<u32>())
        );
    }

    #[test]
    fn const_key_parts_defer_to_const_value_equality() {
        assert_eq!(
            GenericKeyPart::Const(ConstValue::new(5u32)),
            GenericKeyPart::Const(ConstValue::new(5u32))
        );
        assert_ne!(
            GenericKeyPart::Const(ConstValue::new(5u32)),
            GenericKeyPart::Const(ConstValue::new(7u32))
        );
    }

    #[test]
    fn type_and_const_key_parts_are_never_equal() {
        assert_ne!(
            GenericKeyPart::Type(TypeId::of::<u32>()),
            GenericKeyPart::Const(ConstValue::new(5u32))
        );
    }

    #[test]
    fn key_parts_can_be_used_as_hash_set_members() {
        let mut set = HashSet::new();
        set.insert(GenericKeyPart::Type(TypeId::of::<i32>()));
        set.insert(GenericKeyPart::Const(ConstValue::new(5u32)));

        assert!(set.contains(&GenericKeyPart::Type(TypeId::of::<i32>())));
        assert!(set.contains(&GenericKeyPart::Const(ConstValue::new(5u32))));
        assert!(!set.contains(&GenericKeyPart::Type(TypeId::of::<u32>())));
        assert!(!set.contains(&GenericKeyPart::Const(ConstValue::new(7u32))));
    }
}
