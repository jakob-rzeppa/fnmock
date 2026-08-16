pub struct FunctionCommonScheme {
    pub vis: syn::Visibility,

    pub item_fn: syn::ItemFn,

    pub module_name: syn::Ident,
    pub display_name: String,
    pub accessor_name: syn::Ident,

    pub accessor_generic_params: Vec<syn::GenericParam>,
}
