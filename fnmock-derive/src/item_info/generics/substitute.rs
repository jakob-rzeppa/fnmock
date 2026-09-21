//! Rewriting of the lifetimes a parameter's bounds mention into the literal `'static`.
//!
//! A parameter is redeclared verbatim on the items fnmock generates — the accessor function, the
//! store module — and those items live in a scope where the original item's lifetimes do not
//! exist. A bound like `T: 'a` or `T: Into<&'a str>` would therefore fail with `use of undeclared
//! lifetime name`. Every lifetime that is provably `'static` can be spelled `'static` instead,
//! which means the same thing and is in scope everywhere.

use std::collections::HashSet;

use syn::visit_mut::VisitMut;

/// Rewrite every provably-`'static` lifetime in the bounds of `params` to the literal `'static`.
///
/// Only bounds are visited; parameter and return types are left alone, since a fake binds the
/// item's lifetimes higher-ranked on its closure trait rather than redeclaring them.
pub fn substitute_static_lifetimes(
    params: &mut [syn::GenericParam],
    static_lifetimes: &HashSet<String>,
) {
    let mut substituter = SubstituteStaticLifetimes {
        static_lifetimes,
        shadowed: Vec::new(),
    };

    for param in params {
        if let syn::GenericParam::Type(type_param) = param {
            for bound in &mut type_param.bounds {
                substituter.visit_type_param_bound_mut(bound);
            }
        }
    }
}

struct SubstituteStaticLifetimes<'a> {
    static_lifetimes: &'a HashSet<String>,
    /// The lifetimes bound by the `for<..>` binders currently in scope. A binder introduces a
    /// fresh lifetime that shadows anything of the same name outside the bound, so it must not
    /// be rewritten — `for<'a> Fn(&'a str)` means the same thing whatever `'a` denotes outside.
    shadowed: Vec<String>,
}

impl VisitMut for SubstituteStaticLifetimes<'_> {
    fn visit_trait_bound_mut(&mut self, trait_bound: &mut syn::TraitBound) {
        let introduced = trait_bound
            .lifetimes
            .iter()
            .flat_map(|binder| binder.lifetimes.iter())
            .filter_map(|def| match def {
                syn::GenericParam::Lifetime(lifetime_param) => {
                    Some(lifetime_param.lifetime.ident.to_string())
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        let depth = self.shadowed.len();
        self.shadowed.extend(introduced);
        syn::visit_mut::visit_path_mut(self, &mut trait_bound.path);
        self.shadowed.truncate(depth);
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        let name = lifetime.ident.to_string();
        if !self.shadowed.contains(&name) && self.static_lifetimes.contains(&name) {
            lifetime.ident = syn::Ident::new("static", lifetime.ident.span());
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    fn substitute(param: syn::GenericParam, static_names: &[&str]) -> String {
        let names = static_names
            .iter()
            .map(|name| (*name).to_string())
            .chain(["static".to_string()])
            .collect::<HashSet<_>>();
        let mut params = [param];

        substitute_static_lifetimes(&mut params, &names);

        params[0].to_token_stream().to_string()
    }

    fn expect(param: syn::GenericParam) -> String {
        param.to_token_stream().to_string()
    }

    #[test]
    fn test_a_static_outlives_bound_is_rewritten() {
        assert_eq!(
            substitute(syn::parse_quote!(T: 'a), &["a"]),
            expect(syn::parse_quote!(T: 'static))
        );
    }

    #[test]
    fn test_a_non_static_outlives_bound_is_left_alone() {
        assert_eq!(
            substitute(syn::parse_quote!(T: 'a), &[]),
            expect(syn::parse_quote!(T: 'a))
        );
    }

    #[test]
    fn test_an_existing_static_bound_is_unchanged() {
        assert_eq!(
            substitute(syn::parse_quote!(T: 'static), &[]),
            expect(syn::parse_quote!(T: 'static))
        );
    }

    #[test]
    fn test_a_lifetime_inside_a_trait_bound_is_rewritten() {
        assert_eq!(
            substitute(syn::parse_quote!(T: Into<&'a str> + 'static), &["a"]),
            expect(syn::parse_quote!(T: Into<&'static str> + 'static))
        );
    }

    #[test]
    fn test_a_lifetime_argument_of_a_trait_bound_is_rewritten() {
        assert_eq!(
            substitute(syn::parse_quote!(T: AsRef<Cow<'a, str>> + 'static), &["a"]),
            expect(syn::parse_quote!(T: AsRef<Cow<'static, str>> + 'static))
        );
    }

    #[test]
    fn test_a_lifetime_bound_by_the_bounds_own_binder_is_left_alone() {
        assert_eq!(
            substitute(
                syn::parse_quote!(F: for<'a> Fn(&'a str) -> String + 'static),
                &["a"]
            ),
            expect(syn::parse_quote!(F: for<'a> Fn(&'a str) -> String + 'static))
        );
    }

    #[test]
    fn test_a_binder_only_shadows_inside_its_own_bound() {
        assert_eq!(
            substitute(
                syn::parse_quote!(F: for<'a> Fn(&'a str) -> String + Into<&'a str>),
                &["a"]
            ),
            expect(syn::parse_quote!(F: for<'a> Fn(&'a str) -> String + Into<&'static str>))
        );
    }

    #[test]
    fn test_a_const_parameter_is_untouched() {
        assert_eq!(
            substitute(syn::parse_quote!(const N: usize), &["a"]),
            expect(syn::parse_quote!(const N: usize))
        );
    }
}
