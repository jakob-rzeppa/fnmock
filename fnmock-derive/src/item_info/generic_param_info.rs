use quote::{ToTokens, quote};

/// One generic parameter, bundled with everything derived from it.
#[derive(Clone)]
pub struct GenericParamInfo {
    /// The parameter including its bounds (e.g. `T: Display + 'static`), for redeclaring it on
    /// generated items.
    pub param: syn::GenericParam,

    /// Just the parameter's identifier (e.g. `T`), for instantiating generated items.
    pub ident: syn::Ident,

    /// The `GenericKeyPart` expression that keys a store by this parameter.
    ///
    /// Type parameters are keyed by their `TypeId`. Const parameters are keyed by their actual
    /// value (via `fnmock::generic_fake_store::ConstValue::new`), not just the `TypeId` of their
    /// type — otherwise every value of e.g. `const C: usize` would collapse onto the single key
    /// `TypeId::of::<usize>()`.
    ///
    /// The expression is emitted into the generated code, where it is evaluated at the call site
    /// with the generic parameters bound to the arguments the call was made with.
    pub key: syn::Expr,
}

/// Build one [`GenericParamInfo`] per parameter, in declaration order.
///
/// Merges any `where` bounds into the type parameters, so that the `GenericParamInfo` has the full
/// bounds for redeclaring the parameter on generated items.
pub fn extract_generic_param_infos(generics: &syn::Generics) -> syn::Result<Vec<GenericParamInfo>> {
    let mut generic_params = generics
        .params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Type(_) => Some(param.clone()),
            syn::GenericParam::Const(_) => Some(param.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();

    // We merge the where bounds into the type parameters, so that the `GenericParamInfo` has the full bounds for redeclaring the parameter on generated items.
    // This makes it easier to generate the code, because we don't have to worry about where bounds separately.
    merge_where_bounds_into_type_params(generics, &mut generic_params);

    // Check if any of the type parameters have a lifetime bound. If so, we check if it is static. If not we return an error, because we don't support non-static lifetimes in generic parameters for fakeable functions.
    for generic_param in &generic_params {
        if let syn::GenericParam::Type(type_param) = generic_param {
            for bound in &type_param.bounds {
                if let syn::TypeParamBound::Lifetime(lifetime) = bound {
                    if lifetime.ident != "static" {
                        return Err(syn::Error::new_spanned(
                            lifetime,
                            format!(
                                "Non-static lifetime '{}' found in generic parameter '{}'. Only 'static lifetimes are supported in generic parameters for fakeable functions.",
                                lifetime.ident, type_param.ident
                            ),
                        ));
                    }
                }
            }
        }
    }

    Ok(generic_params
        .iter()
        .filter_map(|param| {
            let (ident, key_tokens) = match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = type_param.ident.clone();
                    let key_tokens = quote! {
                        ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<#ident>())
                    };
                    (ident, key_tokens)
                }
                syn::GenericParam::Const(const_param) => {
                    let ident = const_param.ident.clone();
                    let key_tokens = quote! {
                        ::fnmock::generic_fake_store::key::GenericKeyPart::Const(
                            ::fnmock::generic_fake_store::key::ConstValue::new(#ident)
                        )
                    };
                    (ident, key_tokens)
                }
                syn::GenericParam::Lifetime(_) => return None,
            };

            let key = syn::parse2(key_tokens).unwrap_or_else(|e| {
                panic!(
                    "internal error: failed to build a generic key expression: {}. This is a bug in fnmock; please report it.",
                    e
                );
            });

            Some(GenericParamInfo {
                param: param.clone(),
                ident,
                key,
            })
        })
        .collect())
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    fn idents_of(infos: &[GenericParamInfo]) -> Vec<String> {
        infos
            .iter()
            .map(|i| i.ident.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_no_generics_returns_empty_vec() {
        let generics: syn::Generics = syn::parse_quote!();

        let result = extract_generic_param_infos(&generics).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_single_type_param() {
        let generics: syn::Generics = syn::parse_quote!(<T>);

        let result = extract_generic_param_infos(&generics).unwrap();

        assert_eq!(idents_of(&result), vec!["T".to_string()]);
        let expected_key = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<T>())
        };
        assert_eq!(
            result[0].key.to_token_stream().to_string(),
            expected_key.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_type_param_strips_bounds_from_ident() {
        let generics: syn::Generics = syn::parse_quote!(<T: Clone + 'static>);

        let result = extract_generic_param_infos(&generics).unwrap();

        assert_eq!(idents_of(&result), vec!["T".to_string()]);
        assert_eq!(
            result[0].param.to_token_stream().to_string(),
            quote::quote!(T: Clone + 'static).to_string()
        );
    }

    #[test]
    fn test_const_param() {
        let generics: syn::Generics = syn::parse_quote!(<const N: usize>);

        let result = extract_generic_param_infos(&generics).unwrap();

        assert_eq!(idents_of(&result), vec!["N".to_string()]);
        let expected_key = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Const(
                ::fnmock::generic_fake_store::key::ConstValue::new(N)
            )
        };
        assert_eq!(
            result[0].key.to_token_stream().to_string(),
            expected_key.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_mixed_type_and_const_params_preserve_order() {
        let generics: syn::Generics = syn::parse_quote!(<T, const N: usize>);

        let result = extract_generic_param_infos(&generics).unwrap();

        assert_eq!(idents_of(&result), vec!["T".to_string(), "N".to_string()]);

        let expected_type_key = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<T>())
        };
        let expected_const_key = quote! {
            ::fnmock::generic_fake_store::key::GenericKeyPart::Const(
                ::fnmock::generic_fake_store::key::ConstValue::new(N)
            )
        };
        assert_eq!(
            result[0].key.to_token_stream().to_string(),
            expected_type_key.to_token_stream().to_string()
        );
        assert_eq!(
            result[1].key.to_token_stream().to_string(),
            expected_const_key.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_multiple_type_params_preserve_order() {
        let generics: syn::Generics = syn::parse_quote!(<A, B>);

        let result = extract_generic_param_infos(&generics).unwrap();

        assert_eq!(idents_of(&result), vec!["A".to_string(), "B".to_string()]);
    }

    #[test]
    fn test_skips_lifetime_params() {
        let generics: syn::Generics = syn::parse_quote!(<'a, T>);

        let result = extract_generic_param_infos(&generics).unwrap();

        assert_eq!(idents_of(&result), vec!["T".to_string()]);
    }

    #[test]
    fn test_non_static_lifetime_bound_is_rejected() {
        let generics: syn::Generics = syn::parse_quote!(<'a, T: 'a>);

        let result = extract_generic_param_infos(&generics);

        assert!(result.is_err());
    }

    mod merge {
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
}
