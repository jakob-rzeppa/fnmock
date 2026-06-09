use quote::quote;

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a `Generics` object
///
/// Returns a vector of `TypeParam` objects representing the generic type parameters
pub fn extract_generic_type_params(generics: &syn::Generics) -> Vec<syn::TypeParam> {
    generics.params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(type_param) = param {
                Some(type_param.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Extract the generic idents (e.g. `T`, `U`) from a list of generic parameters (e.g. `T: Display + 'static`, `U: 'static`)
pub fn extract_generic_idents_from_params(generic_params: &[syn::TypeParam]) -> Vec<syn::Ident> {
    generic_params
        .iter()
        .map(|param| param.ident.clone())
        .collect()
}

/// Build TypeId array: [TypeId::of::<T>(), TypeId::of::<U>(), ...]
pub fn build_type_id_array(generic_idents: &[syn::Ident]) -> Vec<proc_macro2::TokenStream> {
    generic_idents
        .iter()
        .map(|ident| {
            quote! { std::any::TypeId::of::<#ident>() }
        })
        .collect()
}
