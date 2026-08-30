use crate::item_info::generic_param_info::GenericParamInfo;

pub struct GenericScheme {
    /// The parameters including their bounds (e.g. `T: Display + 'static`), for redeclaring them
    /// on generated items.
    pub params: Vec<syn::GenericParam>,
    /// Just the parameters' identifiers (e.g. `T`), for instantiating generated items.
    pub idents: Vec<syn::Ident>,
    /// `idents` with const generics filtered out (they can't appear inside `PhantomData`).
    pub idents_without_const_generics: Vec<syn::Ident>,
    /// The `GenericKeyPart` expressions that key a store by these generics, in parameter order.
    pub keys: Vec<syn::Expr>,
}

pub fn build_generic_scheme(generic_params: &[GenericParamInfo]) -> Option<GenericScheme> {
    if generic_params.is_empty() {
        return None;
    }

    let params = generic_params.iter().map(|g| g.param.clone()).collect();
    let idents = generic_params
        .iter()
        .map(|g| g.ident.clone())
        .collect::<Vec<_>>();
    let idents_without_const_generics = generic_params
        .iter()
        .filter_map(|g| match &g.param {
            syn::GenericParam::Type(_) => Some(g.ident.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let keys = generic_params
        .iter()
        .map(|g| g.key.clone())
        .collect::<Vec<_>>();

    Some(GenericScheme {
        params,
        idents,
        idents_without_const_generics,
        keys,
    })
}

/// Builds the expression that renders one generic parameter into an instantiation's display
/// name: the full type name for a type parameter (e.g. `"alloc::string::String"`), or the value
/// itself for a const parameter (e.g. `"5"`).
pub fn build_generic_display_fragment(info: &GenericParamInfo) -> syn::Expr {
    let ident = &info.ident;
    match &info.param {
        syn::GenericParam::Const(_) => syn::parse_quote! { #ident.to_string() },
        // Type params, and lifetimes (which `extract_generic_param_infos` never produces a
        // `GenericParamInfo` for in the first place).
        _ => syn::parse_quote! { ::std::any::type_name::<#ident>().to_string() },
    }
}
