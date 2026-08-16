//! Reduction of generic parameters to their bare identifiers.

use syn::parse_quote;

use crate::item_info::generics::sanitized_params::SanitizedGenericParams;

/// Extract the generic idents (e.g. `T`, `U`) from a list of generic parameters (e.g. `T: Display + 'static`, `U: 'static`)
///
/// For `const C: usize` this will extract `C`
///
/// Generated items need the bounded form (`T: Display + 'static`) where they declare the
/// parameters and the bare form (`T`) where they instantiate them; this produces the latter.
///
/// # Errors
///
/// Returns a spanned error if a lifetime parameter is encountered, which would mean the params
/// were not sanitized first. That is a bug in fnmock rather than a user error.
pub fn extract_generic_idents_from_generic_params(
    generic_params: &SanitizedGenericParams,
) -> syn::Result<Vec<syn::Ident>> {
    generic_params
        .get_generic_params()
        .iter()
        .map(|param| {
            match param {
                syn::GenericParam::Type(type_param) => {
                    let type_ident = &type_param.ident;
                    Ok(parse_quote!(#type_ident))
                }
                syn::GenericParam::Const(const_param) => {
                    let const_ident = &const_param.ident;
                    Ok(parse_quote!(#const_ident))
                }
                other =>
                    Err(
                        syn::Error::new_spanned(
                            other,
                            "internal error: expected only type and const parameters when extracting generic idents, but found a lifetime parameter. This is a bug in fnmock; please report it."
                        )
                    ),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    fn to_token_strings(idents: &[syn::Ident]) -> Vec<String> {
        idents
            .iter()
            .map(|i| i.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_extract_generic_types_empty() {
        let params = SanitizedGenericParams::new(vec![]).unwrap();

        let result = extract_generic_idents_from_generic_params(&params).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_generic_types_single_type_param() {
        let type_param: syn::GenericParam = parse_quote!(T);
        let params = SanitizedGenericParams::new(vec![type_param]).unwrap();

        let result = extract_generic_idents_from_generic_params(&params).unwrap();

        assert_eq!(to_token_strings(&result), vec!["T".to_string()]);
    }

    #[test]
    fn test_extract_generic_types_strips_bounds_from_type_param() {
        let type_param: syn::GenericParam = parse_quote!(T: Clone + 'static);
        let params = SanitizedGenericParams::new(vec![type_param]).unwrap();

        let result = extract_generic_idents_from_generic_params(&params).unwrap();

        assert_eq!(to_token_strings(&result), vec!["T".to_string()]);
    }

    #[test]
    fn test_extract_generic_types_const_param() {
        let const_param: syn::GenericParam = parse_quote!(const N: usize);
        let params = SanitizedGenericParams::new(vec![const_param]).unwrap();

        let result = extract_generic_idents_from_generic_params(&params).unwrap();

        assert_eq!(to_token_strings(&result), vec!["N".to_string()]);
    }

    #[test]
    fn test_extract_generic_types_mixed_type_and_const_params_preserves_order() {
        let type_param: syn::GenericParam = parse_quote!(T);
        let const_param: syn::GenericParam = parse_quote!(const N: usize);
        let params = SanitizedGenericParams::new(vec![type_param, const_param]).unwrap();

        let result = extract_generic_idents_from_generic_params(&params).unwrap();

        assert_eq!(
            to_token_strings(&result),
            vec!["T".to_string(), "N".to_string()]
        );
    }
}
