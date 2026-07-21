//! Folding of a where clause back into the generic parameters it constrains.

use quote::ToTokens;

/// Merge the where bounds into the type parameters.
/// This is necessary because the where bounds are not included in the generic parameters, but we need them to generate the fakeable function.
///
/// Generated items redeclare the parameters inline (`<T: Display>`) rather than reproducing the
/// where clause, so `fn f<T>(..) where T: Display` and `fn f<T: Display>(..)` have to arrive at
/// the generators in the same shape. Bounds that are already present are not duplicated, and
/// predicates that constrain something other than a parameter (e.g. `where Vec<T>: Clone`) have
/// nowhere to go and are dropped — they don't affect the parameters themselves.
/// Resolve the parameter a bounded type refers to, if it is a plain reference to one.
///
/// `T` and `(T)` resolve to `T`; `Vec<T>`, `T::Assoc`, `<T as Trait>::X` resolve to `None`.
fn resolve_param_ident(ty: &syn::Type) -> Option<&syn::Ident> {
    match ty {
        syn::Type::Paren(paren) => resolve_param_ident(&paren.elem),
        syn::Type::Group(group) => resolve_param_ident(&group.elem),
        syn::Type::Path(type_path) if type_path.qself.is_none() => {
            let path = &type_path.path;
            let segment = path.segments.first()?;
            (path.leading_colon.is_none()
                && path.segments.len() == 1
                && matches!(segment.arguments, syn::PathArguments::None))
            .then_some(&segment.ident)
        }
        _ => None,
    }
}

pub fn merge_where_bounds_into_type_params(
    generics: &syn::Generics,
    type_params: &mut [syn::GenericParam],
) {
    let Some(where_clause) = &generics.where_clause else {
        return;
    };

    for predicate in &where_clause.predicates {
        let syn::WherePredicate::Type(type_predicate) = predicate else {
            continue;
        };

        let Some(target_ident) = resolve_param_ident(&type_predicate.bounded_ty) else {
            // Non-parameter where bounds can be ignored by the fakeable macro, because they don't affect the generic parameters of the function.
            // For example, `where Vec<T>: Clone` is a non-parameter where bound, because it doesn't affect the generic parameter `T`. We can ignore it and continue.
            continue;
        };

        let Some(syn::GenericParam::Type(type_param)) = type_params.iter_mut().find(
            |param| matches!(param, syn::GenericParam::Type(tp) if tp.ident == *target_ident),
        ) else {
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
    fn test_merge_where_bounds_into_type_params_matches_parenthesized_param() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T>()
            where
                (T): Clone,
            {
            }
        );
        let generics = function.sig.generics;
        let mut type_params = vec![syn::parse_quote!(T)];

        merge_where_bounds_into_type_params(&generics, &mut type_params);

        let expected_type_params: Vec<syn::GenericParam> = vec![syn::parse_quote!(T: Clone)];
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
