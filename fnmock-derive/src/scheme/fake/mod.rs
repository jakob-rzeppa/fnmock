use crate::item_info::call_value::CallValue;

pub mod function;
pub mod impl_block;

pub struct FakeScheme {
    pub store_name: syn::Ident,

    pub fn_closure_trait: syn::TraitBound,

    pub fake_call_values: Vec<CallValue>,
}
