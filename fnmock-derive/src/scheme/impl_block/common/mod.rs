use crate::{item_info::original::OriginalImpl, scheme::common::generic_scheme::GenericScheme};

pub struct ImplCommonScheme {
    pub original: OriginalImpl,
}

pub struct ImplCommonMethodScheme {
    pub vis: syn::Visibility,

    pub accessor_name: syn::Ident,
    pub module_name: syn::Ident,
    pub display_name: String,
    pub interface_name: syn::Ident,

    /// The struct's and method's generics, combined.
    pub generic_scheme: Option<GenericScheme>,
    pub method_generic_params: Vec<syn::GenericParam>,
}
