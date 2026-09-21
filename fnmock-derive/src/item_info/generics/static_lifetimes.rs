//! The lifetimes of an item that are provably `'static`.
//!
//! A type parameter has to be `'static` for fnmock to key its store by `TypeId`, but the bound
//! does not have to be spelled `T: 'static`. `T: 'a` says exactly the same thing whenever `'a`
//! is itself constrained to outlive `'static`, whether that is written on the lifetime's own
//! declaration (`<'a: 'static>`) or as a `where` predicate (`where 'a: 'static`) — and the
//! constraint can arrive through a chain of other lifetimes.

use std::collections::HashSet;

/// The idents (without the leading apostrophe) of every lifetime in `generics` that is provably
/// `'static`, including `'static` itself.
///
/// `'a: 'b` is read as "`'a` outlives `'b`", so `'a` is `'static` as soon as any lifetime it
/// outlives is. The relation is followed transitively but never backwards: `'static: 'a` says
/// nothing about `'a`.
pub fn collect_static_lifetimes(generics: &syn::Generics) -> HashSet<String> {
    let mut outlives: Vec<(String, Vec<String>)> = Vec::new();

    for param in &generics.params {
        if let syn::GenericParam::Lifetime(lifetime_param) = param {
            outlives.push((
                lifetime_param.lifetime.ident.to_string(),
                lifetime_param
                    .bounds
                    .iter()
                    .map(|bound| bound.ident.to_string())
                    .collect(),
            ));
        }
    }

    if let Some(where_clause) = &generics.where_clause {
        for predicate in &where_clause.predicates {
            if let syn::WherePredicate::Lifetime(lifetime_predicate) = predicate {
                outlives.push((
                    lifetime_predicate.lifetime.ident.to_string(),
                    lifetime_predicate
                        .bounds
                        .iter()
                        .map(|bound| bound.ident.to_string())
                        .collect(),
                ));
            }
        }
    }

    // Fixpoint rather than a recursive walk: the relations can be declared in any order and may
    // even be cyclic between two non-'static lifetimes, which a naive traversal would not
    // terminate on.
    let mut known = HashSet::from(["static".to_string()]);
    loop {
        let mut grew = false;
        for (lifetime, bounds) in &outlives {
            if !known.contains(lifetime) && bounds.iter().any(|bound| known.contains(bound)) {
                known.insert(lifetime.clone());
                grew = true;
            }
        }
        if !grew {
            return known;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_of(generics: syn::Generics) -> Vec<String> {
        let mut names = collect_static_lifetimes(&generics)
            .into_iter()
            .collect::<Vec<_>>();
        names.sort();
        names
    }

    #[test]
    fn test_static_itself_is_always_in_the_set() {
        assert_eq!(set_of(syn::parse_quote!(<T>)), vec!["static".to_string()]);
    }

    #[test]
    fn test_an_unconstrained_lifetime_is_not_static() {
        assert_eq!(set_of(syn::parse_quote!(<'a>)), vec!["static".to_string()]);
    }

    #[test]
    fn test_a_lifetime_declared_outliving_static_is_static() {
        assert_eq!(
            set_of(syn::parse_quote!(<'a: 'static>)),
            vec!["a".to_string(), "static".to_string()]
        );
    }

    #[test]
    fn test_a_where_predicate_makes_a_lifetime_static() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<'a>()
            where
                'a: 'static,
            {
            }
        );

        assert_eq!(
            set_of(function.sig.generics),
            vec!["a".to_string(), "static".to_string()]
        );
    }

    #[test]
    fn test_outliving_a_static_lifetime_is_transitive() {
        assert_eq!(
            set_of(syn::parse_quote!(<'a: 'static, 'b: 'a, 'c: 'b>)),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "static".to_string()
            ]
        );
    }

    #[test]
    fn test_transitivity_does_not_depend_on_declaration_order() {
        assert_eq!(
            set_of(syn::parse_quote!(<'c: 'b, 'b: 'a, 'a: 'static>)),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "static".to_string()
            ]
        );
    }

    #[test]
    fn test_the_relation_is_not_followed_backwards() {
        // `'static: 'a` says 'static outlives 'a, which tells us nothing about 'a.
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<'a>()
            where
                'static: 'a,
            {
            }
        );

        assert_eq!(set_of(function.sig.generics), vec!["static".to_string()]);
    }

    #[test]
    fn test_a_cycle_between_two_non_static_lifetimes_terminates() {
        assert_eq!(
            set_of(syn::parse_quote!(<'a: 'b, 'b: 'a>)),
            vec!["static".to_string()]
        );
    }

    #[test]
    fn test_inline_and_where_bounds_combine() {
        let function: syn::ItemFn = syn::parse_quote!(
            fn example<'a: 'static, 'b>()
            where
                'b: 'a,
            {
            }
        );

        assert_eq!(
            set_of(function.sig.generics),
            vec!["a".to_string(), "b".to_string(), "static".to_string()]
        );
    }
}
