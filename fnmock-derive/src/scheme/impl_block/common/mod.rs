use crate::item_info::original::OriginalImpl;

pub struct ImplCommonScheme {
    pub original: OriginalImpl,
}

pub struct ImplCommonMethodScheme {
    pub vis: syn::Visibility,
    pub method_name: syn::Ident,
    pub accessor_name: syn::Ident,
    pub method_generic_params: Vec<syn::GenericParam>,
    pub module_name: syn::Ident,
    pub display_name: String,
}
