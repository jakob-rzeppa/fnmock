//! The two rules a normalized generic parameter has to satisfy.

use syn::visit::Visit;

/// Every type parameter must carry a `'static` bound.
///
/// Both stores are keyed by `TypeId`, which requires `'static`, and a spy's `Expectation<M>`
/// requires `M: Any` on top of that. By the time this runs, any lifetime that was provably
/// `'static` has already been rewritten to the literal `'static`, so a remaining named lifetime
/// really is a non-`'static` one.
pub fn check_static_bounds(params: &[syn::GenericParam]) -> syn::Result<()> {
    for param in params {
        let syn::GenericParam::Type(type_param) = param else {
            continue;
        };

        let mut has_static_bound = false;
        for bound in &type_param.bounds {
            let syn::TypeParamBound::Lifetime(lifetime) = bound else {
                continue;
            };

            if lifetime.ident != "static" {
                return Err(syn::Error::new_spanned(
                    lifetime,
                    format!(
                        "Non-static lifetime '{}' found in generic parameter '{}'. Only 'static lifetimes are supported in generic parameters for fakeable functions.",
                        lifetime.ident, type_param.ident
                    ),
                ));
            }
            has_static_bound = true;
        }

        if !has_static_bound {
            return Err(syn::Error::new_spanned(
                type_param,
                format!(
                    "Generic parameter '{}' has no 'static bound. Type parameters are keyed by TypeId, which requires 'static; add an explicit `{}: 'static` bound.",
                    type_param.ident, type_param.ident
                ),
            ));
        }
    }

    Ok(())
}

/// No bound may name a lifetime that will not be in scope where the bound is redeclared.
///
/// The parameters are redeclared on the accessor and inside the generated store module, neither
/// of which has the original item's lifetimes. `'static` is always in scope, and a lifetime
/// introduced by the bound's own `for<..>` binder travels with it, so those two are fine;
/// anything else would expand to a bare `use of undeclared lifetime name` error pointing into the
/// macro, which is why it is caught here instead.
pub fn check_no_free_lifetimes(params: &[syn::GenericParam]) -> syn::Result<()> {
    for param in params {
        let syn::GenericParam::Type(type_param) = param else {
            continue;
        };

        for bound in &type_param.bounds {
            let syn::TypeParamBound::Trait(trait_bound) = bound else {
                // Outlives bounds are `check_static_bounds`' business.
                continue;
            };

            let mut finder = FindFreeLifetime {
                shadowed: Vec::new(),
                found: None,
            };
            finder.visit_trait_bound(trait_bound);

            if let Some(lifetime) = finder.found {
                return Err(syn::Error::new_spanned(
                    &lifetime,
                    format!(
                        "Non-static lifetime '{}' found in a bound of generic parameter '{}'. fnmock redeclares this bound on the items it generates, where '{} is not in scope. Only 'static lifetimes are supported in the bounds of a generic parameter.",
                        lifetime.ident, type_param.ident, lifetime.ident
                    ),
                ));
            }
        }
    }

    Ok(())
}

/// Finds the first lifetime in a bound that is neither `'static` nor bound by a `for<..>` binder
/// enclosing it.
struct FindFreeLifetime {
    /// The lifetimes bound by the `for<..>` binders currently in scope.
    shadowed: Vec<String>,
    found: Option<syn::Lifetime>,
}

impl Visit<'_> for FindFreeLifetime {
    fn visit_trait_bound(&mut self, trait_bound: &syn::TraitBound) {
        let depth = self.shadowed.len();
        self.shadowed.extend(
            trait_bound
                .lifetimes
                .iter()
                .flat_map(|binder| binder.lifetimes.iter())
                .filter_map(|def| match def {
                    syn::GenericParam::Lifetime(lifetime_param) => {
                        Some(lifetime_param.lifetime.ident.to_string())
                    }
                    _ => None,
                }),
        );
        syn::visit::visit_path(self, &trait_bound.path);
        self.shadowed.truncate(depth);
    }

    fn visit_lifetime(&mut self, lifetime: &syn::Lifetime) {
        if self.found.is_some() {
            return;
        }
        let name = lifetime.ident.to_string();
        if name != "static" && !self.shadowed.contains(&name) {
            self.found = Some(lifetime.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn static_bounds(params: &[syn::GenericParam]) -> syn::Result<()> {
        check_static_bounds(params)
    }

    fn free_lifetimes(params: &[syn::GenericParam]) -> syn::Result<()> {
        check_no_free_lifetimes(params)
    }

    #[test]
    fn test_a_static_bound_is_accepted() {
        assert!(static_bounds(&[syn::parse_quote!(T: 'static)]).is_ok());
    }

    #[test]
    fn test_a_parameter_without_any_lifetime_bound_is_rejected() {
        let error = static_bounds(&[syn::parse_quote!(T: Clone)]).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Generic parameter 'T' has no 'static bound. Type parameters are keyed by TypeId, \
             which requires 'static; add an explicit `T: 'static` bound."
        );
    }

    #[test]
    fn test_a_bare_parameter_is_rejected() {
        assert!(static_bounds(&[syn::parse_quote!(T)]).is_err());
    }

    #[test]
    fn test_a_non_static_lifetime_bound_is_rejected() {
        let error = static_bounds(&[syn::parse_quote!(T: 'a)]).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Non-static lifetime 'a' found in generic parameter 'T'. Only 'static lifetimes are \
             supported in generic parameters for fakeable functions."
        );
    }

    #[test]
    fn test_a_const_parameter_needs_no_static_bound() {
        assert!(static_bounds(&[syn::parse_quote!(const N: usize)]).is_ok());
    }

    #[test]
    fn test_a_bound_naming_no_lifetime_is_accepted() {
        assert!(free_lifetimes(&[syn::parse_quote!(T: Clone + 'static)]).is_ok());
    }

    #[test]
    fn test_a_bound_naming_only_static_is_accepted() {
        assert!(free_lifetimes(&[syn::parse_quote!(T: Into<&'static str> + 'static)]).is_ok());
    }

    #[test]
    fn test_a_bound_naming_a_foreign_lifetime_is_rejected() {
        let error = free_lifetimes(&[syn::parse_quote!(T: Into<&'a str> + 'static)]).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Non-static lifetime 'a' found in a bound of generic parameter 'T'. fnmock \
             redeclares this bound on the items it generates, where 'a is not in scope. Only \
             'static lifetimes are supported in the bounds of a generic parameter."
        );
    }

    #[test]
    fn test_a_lifetime_bound_by_the_bounds_own_binder_is_accepted() {
        assert!(
            free_lifetimes(&[syn::parse_quote!(F: for<'x> Fn(&'x str) -> String + 'static)])
                .is_ok()
        );
    }

    #[test]
    fn test_a_binder_does_not_excuse_a_foreign_lifetime_in_a_sibling_bound() {
        assert!(
            free_lifetimes(&[syn::parse_quote!(F: for<'x> Fn(&'x str) -> String + Into<&'a str>)])
                .is_err()
        );
    }
}
