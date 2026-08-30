//! Removal of every named or anonymous non-`'static` lifetime from a type, leaving it fully
//! elided.

use syn::visit_mut::VisitMut;

/// Drops every non-`'static` lifetime from a `syn::Type`: a reference's own lifetime is removed
/// (`&'a str` -> `&str`), and a lifetime argument in a path's generics is dropped from the
/// argument list entirely (`Ref<'a>` -> `Ref<>`), rather than replaced with the placeholder
/// `'_`.
///
/// A spy's matcher is generated into a module of its own, where the spied function's own
/// lifetime parameters are not in scope, so any that appear in a parameter type — directly
/// (`Ref<'a>`) or nested inside a container (`Vec<&'a str>`) — have to be dealt with somehow.
/// Substituting the placeholder `'_` might seem like the natural fix, but writing `'_` explicitly
/// inside the bound of an argument-position `impl Trait` (as `expectf`'s parameter is) hits an
/// unrelated, unstable Rust restriction ("anonymous lifetimes in `impl Trait` are unstable").
/// Eliding the lifetime by omission instead — `Ref<>` rather than `Ref<'_>` — relies on ordinary
/// lifetime elision and has no such restriction.
pub struct ElideLifetimes;

impl VisitMut for ElideLifetimes {
    fn visit_type_reference_mut(&mut self, type_reference: &mut syn::TypeReference) {
        if let Some(lifetime) = &type_reference.lifetime {
            if lifetime.ident != "static" {
                type_reference.lifetime = None;
            }
        }
        syn::visit_mut::visit_type_reference_mut(self, type_reference);
    }

    fn visit_path_arguments_mut(&mut self, path_arguments: &mut syn::PathArguments) {
        if let syn::PathArguments::AngleBracketed(angle_bracketed) = path_arguments {
            angle_bracketed.args = std::mem::take(&mut angle_bracketed.args)
                .into_iter()
                .filter(|arg| {
                    !matches!(
                        arg,
                        syn::GenericArgument::Lifetime(lifetime) if lifetime.ident != "static"
                    )
                })
                .collect();
        }
        syn::visit_mut::visit_path_arguments_mut(self, path_arguments);
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::visit_mut::VisitMut;

    use super::*;

    fn elide(mut ty: syn::Type) -> String {
        ElideLifetimes.visit_type_mut(&mut ty);
        ty.to_token_stream().to_string()
    }

    #[test]
    fn test_named_lifetime_on_a_reference_is_dropped() {
        let ty: syn::Type = syn::parse_quote!(&'a str);

        assert_eq!(elide(ty), quote::quote!(&str).to_string());
    }

    #[test]
    fn test_elided_lifetime_on_a_reference_is_left_as_is() {
        let ty: syn::Type = syn::parse_quote!(&'_ str);

        assert_eq!(elide(ty), quote::quote!(&str).to_string());
    }

    #[test]
    fn test_named_lifetime_argument_in_a_path_is_dropped_from_the_argument_list() {
        let ty: syn::Type = syn::parse_quote!(Ref<'a>);

        assert_eq!(elide(ty), quote::quote!(Ref<>).to_string());
    }

    #[test]
    fn test_anonymous_lifetime_argument_in_a_path_is_dropped_from_the_argument_list() {
        let ty: syn::Type = syn::parse_quote!(Ref<'_>);

        assert_eq!(elide(ty), quote::quote!(Ref<>).to_string());
    }

    #[test]
    fn test_named_lifetime_nested_in_a_container_is_dropped() {
        let ty: syn::Type = syn::parse_quote!(Vec<&'a str>);

        assert_eq!(elide(ty), quote::quote!(Vec<&str>).to_string());
    }

    #[test]
    fn test_static_lifetime_is_kept() {
        let ty: syn::Type = syn::parse_quote!(&'static str);

        assert_eq!(elide(ty), quote::quote!(&'static str).to_string());
    }

    #[test]
    fn test_static_lifetime_argument_in_a_path_is_kept() {
        let ty: syn::Type = syn::parse_quote!(Ref<'static>);

        assert_eq!(elide(ty), quote::quote!(Ref<'static>).to_string());
    }

    #[test]
    fn test_type_without_a_lifetime_is_unchanged() {
        let ty: syn::Type = syn::parse_quote!(Vec<i32>);

        assert_eq!(elide(ty), quote::quote!(Vec<i32>).to_string());
    }

    #[test]
    fn test_multiple_distinct_lifetimes_are_all_dropped() {
        let ty: syn::Type = syn::parse_quote!((Ref<'a>, Ref<'b>));

        assert_eq!(elide(ty), quote::quote!((Ref<>, Ref<>)).to_string());
    }

    #[test]
    fn test_lifetime_argument_alongside_a_type_argument_is_dropped_but_the_type_stays() {
        let ty: syn::Type = syn::parse_quote!(Cow<'a, str>);

        assert_eq!(elide(ty), quote::quote!(Cow<str>).to_string());
    }
}
