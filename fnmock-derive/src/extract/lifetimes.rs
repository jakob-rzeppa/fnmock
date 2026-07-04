pub fn extract_lifetimes_from_generics(generics: &syn::Generics) -> Vec<syn::Lifetime> {
    generics.params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Lifetime(lifetime_param) = param {
                Some(lifetime_param.lifetime.clone())
            } else {
                None
            }
        })
        .collect()
}
