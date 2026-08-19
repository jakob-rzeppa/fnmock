use std::collections::HashMap;

pub mod fake;
pub mod spy;

pub struct ImplExpandable {
    pub item_impl: syn::ItemImpl,

    pub struct_name: syn::Ident,
    pub struct_generic_params: Vec<syn::GenericParam>,
    pub struct_generic_idents: Vec<syn::Ident>,

    /// The methods in the impl block, keyed by their original names.
    pub methods: HashMap<syn::Ident, ImplMethodExpandable>,
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
