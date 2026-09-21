//! Folding of an item's `where` clause into its generic parameter list.
//!
//! Generated items redeclare the parameters inline (`<T: Display>`) rather than reproducing the
//! `where` clause, so `fn f<T>(..) where T: Display` and `fn f<T: Display>(..)` have to arrive at
//! the generators in the same shape. Predicates that constrain something other than a parameter
//! (`where Vec<T>: Clone`, `where T::Item: Clone`) have nowhere to go and are dropped — they do
//! not constrain the parameter itself, and dropping them is also what keeps any lifetime they
//! mention from leaking onto a generated item.

use quote::ToTokens;

/// The type and const parameters of `generics`, in declaration order, with every `where`
/// predicate that constrains one of them merged into its bounds. Lifetime parameters are not
/// returned; see [`crate::item_info::generics::NormalizedGenerics::lifetimes`].
pub fn merge_where_predicates(generics: &syn::Generics) -> Vec<syn::GenericParam> {
    let mut params = generics
        .params
        .iter()
        .filter(|param| !matches!(param, syn::GenericParam::Lifetime(_)))
        .cloned()
        .collect::<Vec<_>>();

    let Some(where_clause) = &generics.where_clause else {
        return params;
    };

    for predicate in &where_clause.predicates {
        let syn::WherePredicate::Type(type_predicate) = predicate else {
            // Lifetime predicates (`where 'a: 'static`) are read by `collect_static_lifetimes`
            // instead; they constrain no type parameter.
            continue;
        };

        let Some(target) = resolve_param_ident(&type_predicate.bounded_ty) else {
            continue;
        };

        let Some(syn::GenericParam::Type(type_param)) = params.iter_mut().find(
            |param| matches!(param, syn::GenericParam::Type(candidate) if candidate.ident == *target),
        ) else {
            continue;
        };

        for bound in &type_predicate.bounds {
            let bound = bind(bound, type_predicate.lifetimes.as_ref());

            let already_present = type_param.bounds.iter().any(|existing| {
                existing.to_token_stream().to_string() == bound.to_token_stream().to_string()
            });
            if !already_present {
                type_param.bounds.push(bound);
            }
        }
    }

    params
}

/// Re-attach a predicate's own `for<..>` binder to one of its bounds.
///
/// `where for<'x> T: Fn(&'x str)` binds `'x` on the predicate, but a merged bound is written on
/// the parameter, where that binder no longer exists. `syn::TraitBound` carries a binder of its
/// own, so the two spellings are interchangeable: the bound becomes
/// `T: for<'x> Fn(&'x str)`. Without this, `'x` would be left free and every generated item that
/// redeclares the parameter would fail with `use of undeclared lifetime name`.
fn bind(bound: &syn::TypeParamBound, binder: Option<&syn::BoundLifetimes>) -> syn::TypeParamBound {
    let (Some(binder), syn::TypeParamBound::Trait(trait_bound)) = (binder, bound) else {
        return bound.clone();
    };

    let mut trait_bound = trait_bound.clone();
    match &mut trait_bound.lifetimes {
        // The bound has a binder of its own (`for<'x> T: for<'y> Fn(..)`); both sets are in
        // scope inside the bound, so they are unioned rather than one replacing the other.
        Some(existing) => existing.lifetimes.extend(binder.lifetimes.iter().cloned()),
        None => trait_bound.lifetimes = Some(binder.clone()),
    }
    syn::TypeParamBound::Trait(trait_bound)
}

/// Resolve the parameter a bounded type refers to, if it is a plain reference to one.
///
/// `T` and `(T)` resolve to `T`; `Vec<T>`, `T::Assoc` and `<T as Trait>::X` resolve to `None`.
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
    use quote::ToTokens;

    use super::*;

    fn merged(function: syn::ItemFn) -> Vec<String> {
        merge_where_predicates(&function.sig.generics)
            .iter()
            .map(|param| param.to_token_stream().to_string())
            .collect()
    }

    fn expected(params: &[syn::GenericParam]) -> Vec<String> {
        params
            .iter()
            .map(|param| param.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_a_where_bound_is_merged_into_the_parameter() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T>()
            where
                T: Clone + 'static,
            {
            }
        );

        assert_eq!(
            merged(function),
            expected(&[syn::parse_quote!(T: Clone + 'static)])
        );
    }

    #[test]
    fn test_a_parenthesized_bounded_type_still_resolves_to_the_parameter() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T>()
            where
                (T): Clone,
            {
            }
        );

        assert_eq!(merged(function), expected(&[syn::parse_quote!(T: Clone)]));
    }

    #[test]
    fn test_a_bound_already_present_is_not_duplicated() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T: Clone>()
            where
                T: Clone,
            {
            }
        );

        assert_eq!(merged(function), expected(&[syn::parse_quote!(T: Clone)]));
    }

    #[test]
    fn test_a_non_parameter_predicate_is_dropped() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T>()
            where
                Vec<T>: Clone,
            {
            }
        );

        assert_eq!(merged(function), expected(&[syn::parse_quote!(T)]));
    }

    #[test]
    fn test_an_associated_type_predicate_is_dropped() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<T: Iterator>()
            where
                T::Item: Clone,
            {
            }
        );

        assert_eq!(
            merged(function),
            expected(&[syn::parse_quote!(T: Iterator)])
        );
    }

    #[test]
    fn test_lifetime_parameters_are_not_returned() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<'a, T, const N: usize>() {}
        );

        assert_eq!(
            merged(function),
            expected(&[syn::parse_quote!(T), syn::parse_quote!(const N: usize)])
        );
    }

    #[test]
    fn test_a_predicate_binder_is_attached_to_the_merged_bound() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<F>()
            where
                for<'x> F: Fn(&'x str) -> String,
            {
            }
        );

        assert_eq!(
            merged(function),
            expected(&[syn::parse_quote!(F: for<'x> Fn(&'x str) -> String)])
        );
    }

    #[test]
    fn test_a_predicate_binder_is_attached_to_every_bound_of_the_predicate() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<F>()
            where
                for<'x> F: Fn(&'x str) -> String + AsRef<str>,
            {
            }
        );

        assert_eq!(
            merged(function),
            expected(&[syn::parse_quote!(F: for<'x> Fn(&'x str) -> String + for<'x> AsRef<str>)])
        );
    }

    #[test]
    fn test_a_binder_already_on_the_bound_is_kept_and_extended() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<F>()
            where
                for<'x> F: for<'y> Fn(&'x str, &'y str),
            {
            }
        );

        assert_eq!(
            merged(function),
            expected(&[syn::parse_quote!(F: for<'y, 'x> Fn(&'x str, &'y str))])
        );
    }

    #[test]
    fn test_a_lifetime_bound_in_a_predicate_is_merged_as_is() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<'a, T>()
            where
                T: 'a,
            {
            }
        );

        assert_eq!(merged(function), expected(&[syn::parse_quote!(T: 'a)]));
    }
}
