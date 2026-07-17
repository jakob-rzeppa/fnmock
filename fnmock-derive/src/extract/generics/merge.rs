use quote::ToTokens;

/// Merge the where bounds into the type parameters.
/// This is necessary because the where bounds are not included in the generic parameters, but we need them to generate the fakeable function.
pub fn merge_where_bounds_into_type_params(
    generics: &syn::Generics,
    type_params: &mut Vec<syn::GenericParam>,
) {
    let Some(where_clause) = &generics.where_clause else {
        return;
    };

    for predicate in &where_clause.predicates {
        let syn::WherePredicate::Type(type_predicate) = predicate else {
            continue;
        };

        let type_param = if let Some(syn::GenericParam::Type(existing)) =
            type_params.iter_mut().find(|param| match param {
                syn::GenericParam::Type(type_param) => {
                    type_param.ident.to_token_stream().to_string()
                        == type_predicate.bounded_ty.to_token_stream().to_string()
                }
                _ => false,
            }) {
            // If the type parameter already exists, use the existing one
            existing
        } else {
            // Non-parameter where bounds can be ignored by the fakeable macro, because they don't affect the generic parameters of the function.
            // For example, `where Vec<T>: Clone` is a non-parameter where bound, because it doesn't affect the generic parameter `T`. We can ignore it and continue.
            continue;
        };

        for bound in &type_predicate.bounds {
            if type_param.bounds.iter().any(|existing| {
                existing.to_token_stream().to_string() == bound.to_token_stream().to_string()
            }) {
                continue;
            }

            type_param.bounds.push(bound.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_where_bounds_into_type_params_merges_bounds() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T>()
            where
                T: Clone + 'static,
            {
            }
        );
        let generics = function.sig.generics;
        let mut type_params = vec![syn::parse_quote!(T)];

        merge_where_bounds_into_type_params(&generics, &mut type_params);

        let expected_type_params: Vec<syn::GenericParam> =
            vec![syn::parse_quote!(T: Clone + 'static)];
        assert_eq!(
            type_params[0].to_token_stream().to_string(),
            expected_type_params[0].to_token_stream().to_string()
        );
    }

    #[test]
    fn test_merge_where_bounds_into_type_params_ignores_non_parameter_bounds() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T>()
            where
                Vec<T>: Clone,
            {
            }
        );
        let generics = function.sig.generics;
        let type_params: Vec<syn::GenericParam> = vec![syn::parse_quote!(T)];

        merge_where_bounds_into_type_params(&generics, &mut type_params.clone());

        let expected_type_params: Vec<syn::GenericParam> = vec![syn::parse_quote!(T)];
        assert_eq!(
            type_params[0].to_token_stream().to_string(),
            expected_type_params[0].to_token_stream().to_string()
        );
    }

    #[test]
    fn test_merge_where_bounds_into_type_params_ignores_duplicate_bounds() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T: Clone>()
            where
                T: Clone,
            {
            }
        );
        let generics = function.sig.generics;
        let mut type_params = vec![syn::parse_quote!(T: Clone)];

        merge_where_bounds_into_type_params(&generics, &mut type_params);

        let expected_type_params: Vec<syn::GenericParam> = vec![syn::parse_quote!(T: Clone)];
        assert_eq!(
            type_params[0].to_token_stream().to_string(),
            expected_type_params[0].to_token_stream().to_string()
        );
    }
}
