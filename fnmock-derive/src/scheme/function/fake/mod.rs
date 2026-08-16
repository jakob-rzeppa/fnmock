use crate::{
    item_info::{call_value::CallValue, function::info::FunctionInfo},
    scheme::function::common::FunctionCommonScheme,
};

pub struct FunctionFakeScheme {
    pub common: FunctionCommonScheme,

    pub store_name: syn::Ident,

    pub fn_closure_trait: syn::TraitBound,

    pub interface_name: syn::Ident,
    pub interface_type: syn::Type,
    pub fake_call_values: Vec<CallValue>,

    pub generic_count: Option<usize>,
    pub generic_params: Option<Vec<syn::GenericParam>>,
    pub generic_idents: Option<Vec<syn::Ident>>,
    pub generic_idents_without_const_generics: Option<Vec<syn::Ident>>,
    pub generic_keys: Option<Vec<syn::Expr>>,
}

impl TryFrom<FunctionInfo> for FunctionFakeScheme {
    type Error = syn::Error;

    fn try_from(value: FunctionInfo) -> Result<Self, Self::Error> {
        todo!()
    }
}
