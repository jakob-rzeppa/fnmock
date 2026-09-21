//! Normalization of an item's generics into the canonical form the generators consume.
//!
//! Every generated item — the accessor, the store module, the closure trait a fake must satisfy —
//! redeclares the item's type and const parameters inline, in a scope that has none of the item's
//! own lifetimes. Normalizing means getting the parameters into a shape that survives that move:
//! `where` predicates folded in (binders and all), every provably-`'static` lifetime spelled
//! `'static`, and nothing left over that would not be in scope.

pub mod merge;
pub mod static_lifetimes;
pub mod substitute;
pub mod validate;

use std::collections::HashSet;

use quote::quote;

/// One generic parameter, bundled with everything derived from it.
#[derive(Clone)]
pub struct GenericParamInfo {
    /// The parameter including its normalized bounds (e.g. `T: Display + 'static`), for
    /// redeclaring it on generated items.
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

/// An item's generics, taken apart into the three things the generators need.
pub struct NormalizedGenerics {
    /// The type and const parameters, in declaration order, with normalized bounds.
    pub params: Vec<GenericParamInfo>,

    /// The lifetime parameters, in declaration order. Only a fake needs these, to bind them
    /// higher-ranked on its closure trait: a fake is a single stored `dyn Fn` value and cannot be
    /// generic over lifetimes the way the faked item is, so `for<'a> Fn(&'a str)` lets one stored
    /// closure serve calls at any lifetime.
    pub lifetimes: Vec<syn::Lifetime>,

    /// The lifetimes that were proved `'static` while normalizing, so that an inner scope (an
    /// impl block's method) can start from what the outer scope already knows.
    pub static_lifetimes: HashSet<String>,
}

/// Normalize `generics`, rejecting anything fnmock cannot reproduce on a generated item.
///
/// `outer_static_lifetimes` carries down what an enclosing scope already proved `'static`. A
/// method's own `syn::Generics` does not redeclare the impl block's lifetimes, so without this an
/// impl-level `where 'a: 'static` would be invisible to a method that writes `T: 'a`. Free
/// functions pass an empty set.
pub fn normalize_generics(
    generics: &syn::Generics,
    outer_static_lifetimes: &HashSet<String>,
) -> syn::Result<NormalizedGenerics> {
    let mut static_lifetimes = static_lifetimes::collect_static_lifetimes(generics);
    static_lifetimes.extend(outer_static_lifetimes.iter().cloned());

    let mut params = merge::merge_where_predicates(generics);
    substitute::substitute_static_lifetimes(&mut params, &static_lifetimes);

    // Order matters: a plain `T: 'a` should report the outlives message, not the bound message,
    // so the `'static` rule is applied before the free-lifetime audit.
    validate::check_static_bounds(&params)?;
    validate::check_no_free_lifetimes(&params)?;

    let params = params
        .into_iter()
        .map(build_param_info)
        .collect::<syn::Result<Vec<_>>>()?;

    let lifetimes = generics
        .params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Lifetime(lifetime_param) => Some(lifetime_param.lifetime.clone()),
            _ => None,
        })
        .collect();

    Ok(NormalizedGenerics {
        params,
        lifetimes,
        static_lifetimes,
    })
}

fn build_param_info(param: syn::GenericParam) -> syn::Result<GenericParamInfo> {
    let (ident, key_tokens) = match &param {
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
        syn::GenericParam::Lifetime(lifetime_param) => {
            return Err(syn::Error::new_spanned(
                lifetime_param,
                "A lifetime parameter reached the generic key builder. This is an error in fnmock. Please report this bug.",
            ));
        }
    };

    let key = syn::parse2(key_tokens).map_err(|error| {
        syn::Error::new_spanned(
            &param,
            format!(
                "Failed to build a generic key expression: {error}. This is an error in fnmock. Please report this bug."
            ),
        )
    })?;

    Ok(GenericParamInfo { param, ident, key })
}

#[cfg(test)]
mod tests {
    use quote::{ToTokens, quote};

    use super::*;

    fn normalize(generics: syn::Generics) -> syn::Result<NormalizedGenerics> {
        normalize_generics(&generics, &HashSet::new())
    }

    fn idents_of(normalized: &NormalizedGenerics) -> Vec<String> {
        normalized
            .params
            .iter()
            .map(|info| info.ident.to_string())
            .collect()
    }

    fn params_of(normalized: &NormalizedGenerics) -> Vec<String> {
        normalized
            .params
            .iter()
            .map(|info| info.param.to_token_stream().to_string())
            .collect()
    }

    fn expect(param: syn::GenericParam) -> Vec<String> {
        vec![param.to_token_stream().to_string()]
    }

    #[test]
    fn test_no_generics_yields_no_params_and_no_lifetimes() {
        let normalized = normalize(syn::parse_quote!()).unwrap();

        assert!(normalized.params.is_empty());
        assert!(normalized.lifetimes.is_empty());
    }

    #[test]
    fn test_a_type_param_is_keyed_by_type_id() {
        let normalized = normalize(syn::parse_quote!(<T: 'static>)).unwrap();

        assert_eq!(idents_of(&normalized), vec!["T".to_string()]);
        assert_eq!(
            normalized.params[0].key.to_token_stream().to_string(),
            quote! {
                ::fnmock::generic_fake_store::key::GenericKeyPart::Type(::std::any::TypeId::of::<T>())
            }
            .to_string()
        );
    }

    #[test]
    fn test_a_const_param_is_keyed_by_value() {
        let normalized = normalize(syn::parse_quote!(<const N: usize>)).unwrap();

        assert_eq!(idents_of(&normalized), vec!["N".to_string()]);
        assert_eq!(
            normalized.params[0].key.to_token_stream().to_string(),
            quote! {
                ::fnmock::generic_fake_store::key::GenericKeyPart::Const(
                    ::fnmock::generic_fake_store::key::ConstValue::new(N)
                )
            }
            .to_string()
        );
    }

    #[test]
    fn test_params_keep_their_declaration_order() {
        let normalized =
            normalize(syn::parse_quote!(<'a, T: 'static, const N: usize, U: 'static>)).unwrap();

        assert_eq!(
            idents_of(&normalized),
            vec!["T".to_string(), "N".to_string(), "U".to_string()]
        );
    }

    #[test]
    fn test_lifetimes_are_collected_in_declaration_order() {
        let normalized = normalize(syn::parse_quote!(<'a, T: 'static, 'b>)).unwrap();

        assert_eq!(
            normalized
                .lifetimes
                .iter()
                .map(|lifetime| lifetime.to_token_stream().to_string())
                .collect::<Vec<_>>(),
            vec!["'a".to_string(), "'b".to_string()]
        );
    }

    #[test]
    fn test_a_where_bound_reaches_the_redeclared_param() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T>()
            where
                T: Clone + 'static,
            {
            }
        );

        let normalized = normalize(function.sig.generics).unwrap();

        assert_eq!(
            params_of(&normalized),
            expect(syn::parse_quote!(T: Clone + 'static))
        );
    }

    #[test]
    fn test_a_static_named_lifetime_bound_is_accepted_and_normalized() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<'a, T: 'a>()
            where
                'a: 'static,
            {
            }
        );

        let normalized = normalize(function.sig.generics).unwrap();

        assert_eq!(
            params_of(&normalized),
            expect(syn::parse_quote!(T: 'static))
        );
    }

    #[test]
    fn test_a_transitively_static_lifetime_bound_is_accepted() {
        let normalized = normalize(syn::parse_quote!(<'a: 'static, 'b: 'a, T: 'b>)).unwrap();

        assert_eq!(
            params_of(&normalized),
            expect(syn::parse_quote!(T: 'static))
        );
    }

    #[test]
    fn test_a_predicate_binder_survives_normalization() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<F: 'static>()
            where
                for<'x> F: Fn(&'x str) -> String,
            {
            }
        );

        let normalized = normalize(function.sig.generics).unwrap();

        assert_eq!(
            params_of(&normalized),
            expect(syn::parse_quote!(F: 'static + for<'x> Fn(&'x str) -> String))
        );
    }

    #[test]
    fn test_a_non_static_lifetime_bound_is_rejected() {
        assert!(normalize(syn::parse_quote!(<'a, T: 'a>)).is_err());
    }

    #[test]
    fn test_a_param_without_a_static_bound_is_rejected() {
        assert!(normalize(syn::parse_quote!(<T>)).is_err());
    }

    #[test]
    fn test_a_bound_naming_a_non_static_lifetime_is_rejected() {
        assert!(normalize(syn::parse_quote!(<'a, T: Into<&'a str> + 'static>)).is_err());
    }

    #[test]
    fn test_an_outer_static_lifetime_is_honoured() {
        let outer = HashSet::from(["a".to_string(), "static".to_string()]);
        let generics: syn::Generics = syn::parse_quote!(<T: 'a>);

        let normalized = normalize_generics(&generics, &outer).unwrap();

        assert_eq!(
            params_of(&normalized),
            expect(syn::parse_quote!(T: 'static))
        );
    }
}
