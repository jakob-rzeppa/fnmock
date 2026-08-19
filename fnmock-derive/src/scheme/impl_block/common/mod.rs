pub struct ImplCommonScheme {
    pub item_impl: syn::ItemImpl,
    pub struct_name: syn::Ident,
    pub struct_generic_params: Vec<syn::GenericParam>,
    pub struct_generic_idents: Vec<syn::Ident>,
}

pub struct ImplCommonMethodScheme {
    pub vis: syn::Visibility,
    pub method_name: syn::Ident,
    pub accessor_name: syn::Ident,
    pub method_generic_params: Vec<syn::GenericParam>,
    pub module_name: syn::Ident,
    pub display_name: String,
}
