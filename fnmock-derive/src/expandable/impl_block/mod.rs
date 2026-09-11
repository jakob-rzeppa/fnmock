use crate::item_info::original::OriginalImpl;

pub mod fake;
pub mod spy;

pub struct ImplExpandable {
    pub original: OriginalImpl,

    /// (method_name, method_info) - The methods in the impl block.
    /// The order of the methods must be preserved from the original impl block.
    pub methods: Vec<(syn::Ident, ImplMethodExpandable)>,
}

pub struct ImplMethodExpandable {
    pub vis: syn::Visibility,

    pub inline_call: syn::Block,

    pub accessor_name: syn::Ident,

    pub method_generic_params: Vec<syn::GenericParam>,

    /// The type with generics
    pub interface_type: syn::Type,

    pub module_name: syn::Ident,
    pub module_parts: Vec<proc_macro2::TokenStream>,
}
