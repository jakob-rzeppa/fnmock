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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    fn lifetime_names(lifetimes: &[syn::Lifetime]) -> Vec<String> {
        lifetimes
            .iter()
            .map(|lifetime| lifetime.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_no_generics_returns_empty_vec() {
        let generics: syn::Generics = syn::parse_quote!(<>);

        let lifetimes = extract_lifetimes_from_generics(&generics);

        assert!(lifetimes.is_empty());
    }

    #[test]
    fn test_only_lifetimes_returns_all_in_order() {
        let generics: syn::Generics = syn::parse_quote!(<'a, 'b>);

        let lifetimes = extract_lifetimes_from_generics(&generics);

        assert_eq!(lifetime_names(&lifetimes), vec!["'a".to_string(), "'b".to_string()]);
    }

    #[test]
    fn test_mixed_lifetime_type_and_const_returns_only_lifetime() {
        let generics: syn::Generics = syn::parse_quote!(<'a, T, const N: usize>);

        let lifetimes = extract_lifetimes_from_generics(&generics);

        assert_eq!(lifetime_names(&lifetimes), vec!["'a".to_string()]);
    }

    #[test]
    fn test_lifetimes_interleaved_with_type_returns_in_order() {
        let generics: syn::Generics = syn::parse_quote!(<'a, T, 'b>);

        let lifetimes = extract_lifetimes_from_generics(&generics);

        assert_eq!(lifetime_names(&lifetimes), vec!["'a".to_string(), "'b".to_string()]);
    }
}
