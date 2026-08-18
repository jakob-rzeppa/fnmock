use crate::{
    item_info::{call_value::CallValue, impl_block::info::ImplBlockInfo},
    scheme::impl_block::common::{ImplCommonMethodScheme, ImplCommonScheme},
};

pub struct ImplFakeScheme {
    pub common: ImplCommonScheme,

    pub methods: Vec<ImplFakeMethodScheme>,
}

pub struct ImplFakeMethodScheme {
    pub common: ImplCommonMethodScheme,

    pub store_name: syn::Ident,

    pub fn_closure_trait: syn::TraitBound,

    pub interface_name: syn::Ident,
    pub interface_type: syn::Type,
    pub fake_call_values: Vec<CallValue>,

    /// The struct's and method's generics, combined.
    pub generic_count: Option<usize>,
    pub generic_params: Option<Vec<syn::GenericParam>>,
    pub generic_idents: Option<Vec<syn::Ident>>,
    pub generic_idents_without_const_generics: Option<Vec<syn::Ident>>,
    pub generic_keys: Option<Vec<syn::Expr>>,
}

impl TryFrom<ImplBlockInfo> for ImplFakeScheme {
    type Error = syn::Error;

    fn try_from(value: ImplBlockInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
