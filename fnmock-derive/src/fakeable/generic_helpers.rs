use quote::quote;

pub fn generate_fake_store_name(func_name: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("{}_FAKE", func_name.to_string().to_uppercase()), func_name.span())
}

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
