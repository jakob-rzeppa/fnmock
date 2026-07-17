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
