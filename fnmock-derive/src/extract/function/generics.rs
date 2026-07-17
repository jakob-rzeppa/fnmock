use crate::extract::{
    function::info::FunctionGenericInfo,
    generics::{
        key_array::build_generic_key_array,
        params::extract_generic_type_and_const_params,
        types::extract_generic_itents_from_generic_params,
    },
};

/// Extracts the generic information from a `Generics` object, including the count of generic parameters, the generic type parameters themselves, their identifiers, and their corresponding `GenericKeyPart` expressions.
///
/// This is used for free functions and not impl blocks, as impl blocks require special handling to combine the generic parameters from both the struct and the method.
pub fn extract_generic_function_info(
    generics: &syn::Generics
) -> syn::Result<Option<FunctionGenericInfo>> {
    let generic_params = extract_generic_type_and_const_params(generics)?;

    if generic_params.is_empty() {
        return Ok(None);
    }

    let idents = extract_generic_itents_from_generic_params(&generic_params)?;
    let generic_keys = build_generic_key_array(&generic_params)?;

    Ok(
        Some(FunctionGenericInfo {
            count: generic_params.len(),
            generic_params: generic_params.to_generic_params(),
            idents,
            generic_keys,
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    fn as_strings<T: ToTokens>(items: &[T]) -> Vec<String> {
        items
            .iter()
            .map(|i| i.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_no_generics_returns_none() {
        let generics: syn::Generics = syn::parse_quote!();

        let result = extract_generic_function_info(&generics);

        let Ok(None) = result else {
            panic!("expected Ok(None) for no generics");
        };
    }

    #[test]
    fn test_only_lifetime_returns_none() {
        let generics: syn::Generics = syn::parse_quote!(<'a>);

        let result = extract_generic_function_info(&generics);

        let Ok(None) = result else {
            panic!("expected Ok(None) for a lifetime-only generics list");
        };
    }

    #[test]
    fn test_single_type_param() {
        let generics: syn::Generics = syn::parse_quote!(<T>);

        let result = extract_generic_function_info(&generics);

        let Ok(Some(info)) = result else {
            panic!("expected Ok(Some(_)) for a single type param");
        };
        assert_eq!(info.count, 1);
        assert_eq!(as_strings(&info.idents), vec!["T"]);
    }

    #[test]
    fn test_type_and_const_params() {
        let generics: syn::Generics = syn::parse_quote!(<T, const N: usize>);

        let result = extract_generic_function_info(&generics);

        let Ok(Some(info)) = result else {
            panic!("expected Ok(Some(_)) for type + const params");
        };
        assert_eq!(info.count, 2);
        assert_eq!(as_strings(&info.idents), vec!["T", "N"]);
    }

    #[test]
    fn test_non_static_lifetime_bound_is_err() {
        let generics: syn::Generics = syn::parse_quote!(<T: 'a>);

        let result = extract_generic_function_info(&generics);

        assert!(result.is_err(), "expected a non-static lifetime bound to be an error");
    }
}
