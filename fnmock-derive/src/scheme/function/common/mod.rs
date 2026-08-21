use crate::{item_info::original::OriginalFn, scheme::common::generic_scheme::GenericScheme};

pub struct FunctionCommonScheme {
    pub vis: syn::Visibility,

    pub original: OriginalFn,

    pub module_name: syn::Ident,
    pub display_name: String,
    pub accessor_name: syn::Ident,

    pub interface_name: syn::Ident,

    pub generic_scheme: Option<GenericScheme>,
}
