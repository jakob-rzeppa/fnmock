pub mod fake;
pub mod spy;

pub struct FunctionExpandable {
    pub vis: syn::Visibility,

    pub item_fn: syn::ItemFn,
    pub inline_call: syn::Block,

    pub accessor_name: syn::Ident,
    pub accessor_generic_params: Vec<syn::GenericParam>,

    /// The type with generics
    pub interface_type: syn::Type,

    pub module_name: syn::Ident,
    pub module_parts: Vec<proc_macro2::TokenStream>,
}
