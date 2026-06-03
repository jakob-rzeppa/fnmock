use quote::quote;

pub fn extract_generic_params(generics: &syn::Generics) -> Vec<syn::TypeParam> {
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

pub fn extract_generic_idents(generic_params: &[syn::TypeParam]) -> Vec<syn::Ident> {
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

pub fn check_if_generics_are_all_static(generic_params: &[syn::TypeParam]) -> syn::Result<()> {
    let all_static = generic_params.iter().all(|param| {
        param.bounds.iter().any(|bound| {
            if let syn::TypeParamBound::Lifetime(lifetime) = bound {
                lifetime.ident == "static"
            } else {
                false
            }
        })
    });

    if !all_static {
        return Err(
            syn::Error::new_spanned(
                &generic_params[0],
                "All generic parameters must have a 'static lifetime bound to be used with #[fakeable]"
            )
        );
    }

    Ok(())
}
