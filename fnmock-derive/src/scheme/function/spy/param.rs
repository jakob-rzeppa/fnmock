//! Turning a spied function's parameter type into the type its matcher names it under, and its
//! pattern back into the expression the injected call forwards to the spy.

use syn::visit::Visit;
use syn::visit_mut::VisitMut;

use crate::item_info::elide_lifetimes::ElideLifetimes;

/// Like [`spy_param_type`], but also reports whether the type named a lifetime (other than
/// `'static`) before it was elided; see [`type_needs_a_lifetime_outside_fn_sugar`].
pub fn spy_param_type_with_lifetime_info(ty: &syn::Type) -> (syn::Type, bool) {
    let mut stripped = strip_one_reference(ty);
    let needs_lifetime = type_needs_a_lifetime_outside_fn_sugar(&stripped);
    ElideLifetimes.visit_type_mut(&mut stripped);
    (stripped, needs_lifetime)
}

/// Like [`spy_param_type`], but for the element type of the matcher's `Params<'a>` tuple:
/// substitutes `lifetime` for every non-`'static` lifetime in the type, rather than eliding it
/// by omission.
///
/// `Params<'a>` is a plain associated-type alias, not a `Fn(..) -> ..` trait's argument list, so
/// it gets none of the implicit higher-ranked elision that lets [`spy_param_type`]'s `Ref<>`
/// (lifetime omitted) work inside `expectf`'s closure bound — a type alias demands an explicit
/// lifetime for every slot. Reusing the tuple's own `'a` is sound because a spied parameter type
/// is always covariant in the lifetime it borrows (it can only ever be a reference, or a type
/// like `Ref<'a>` wrapping one), so the real, possibly-longer lifetime a call was made with
/// always coerces down to match `'a`.
pub fn spy_param_type_for_params_tuple(ty: &syn::Type, lifetime: &syn::Lifetime) -> syn::Type {
    let mut stripped = strip_one_reference(ty);
    SubstituteLifetimes { lifetime }.visit_type_mut(&mut stripped);
    stripped
}

fn strip_one_reference(ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Reference(type_reference) => type_reference.elem.as_ref().clone(),
        other => other.clone(),
    }
}

/// Replaces every non-`'static` lifetime in a `syn::Type` with `lifetime`.
struct SubstituteLifetimes<'a> {
    lifetime: &'a syn::Lifetime,
}

impl VisitMut for SubstituteLifetimes<'_> {
    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if lifetime.ident != "static" {
            *lifetime = self.lifetime.clone();
        }
    }
}

/// Whether a (post-[`spy_param_type`]-stripping, pre-elision) type still needs a lifetime
/// argument once elided — i.e. whether it names a lifetime anywhere other than `'static`.
///
/// Eliding a lifetime by omission (`Ref<'a>` -> `Ref<>`) only actually works directly inside a
/// `Fn(..) -> ..` trait's own argument list — that specific position has its own implicit
/// higher-ranked elision built into the language. Everywhere else a matcher needs to name the
/// type (a struct field, an associated type, a plain generic argument like `Predicate<Ref<>>`),
/// an omitted lifetime is a hard error. So a parameter whose type still needs one after
/// elision can only ever be matched by `expectf` — `expect`'s `Predicate<..>`-based matching has
/// no lifetime to give it, and is left off the matcher entirely for that function.
pub fn type_needs_a_lifetime_outside_fn_sugar(ty: &syn::Type) -> bool {
    struct HasNonStaticLifetime(bool);

    impl<'ast> Visit<'ast> for HasNonStaticLifetime {
        fn visit_lifetime(&mut self, lifetime: &'ast syn::Lifetime) {
            if lifetime.ident != "static" {
                self.0 = true;
            }
        }
    }

    let mut visitor = HasNonStaticLifetime(false);
    visitor.visit_type(ty);
    visitor.0
}

/// Builds the expression that passes a parameter on by shared reference to `internal_record_call`,
/// so that its type lines up with [`spy_param_type`]'s.
///
/// - `&T`: forwarded as-is (`id`) — a shared reference is `Copy`, so the binding stays usable.
/// - `&mut T`: reborrowed (`&*id`). Forwarding it as-is would *move* the `&mut` out of the
///   binding and leave the rest of the user's body unable to use the parameter.
/// - anything else: borrowed (`&id`).
pub fn build_reference_call_value(ident: &syn::Ident, ty: &syn::Type) -> syn::Expr {
    match ty {
        syn::Type::Reference(type_reference) if type_reference.mutability.is_some() => {
            syn::parse_quote!(&*#ident)
        }
        syn::Type::Reference(_) => syn::parse_quote!(#ident),
        _ => syn::parse_quote!(&#ident),
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    mod spy_param_type_for_params_tuple_tests {
        use super::*;

        fn lifetime_a() -> syn::Lifetime {
            syn::parse_quote!('a)
        }

        #[test]
        fn test_type_without_a_lifetime_is_unchanged() {
            let ty: syn::Type = syn::parse_quote!(String);

            assert_eq!(
                spy_param_type_for_params_tuple(&ty, &lifetime_a())
                    .to_token_stream()
                    .to_string(),
                quote::quote!(String).to_string()
            );
        }

        #[test]
        fn test_named_lifetime_argument_is_substituted_not_elided() {
            let ty: syn::Type = syn::parse_quote!(Ref<'x>);

            assert_eq!(
                spy_param_type_for_params_tuple(&ty, &lifetime_a())
                    .to_token_stream()
                    .to_string(),
                quote::quote!(Ref<'a>).to_string()
            );
        }

        #[test]
        fn test_elided_lifetime_argument_is_substituted() {
            let ty: syn::Type = syn::parse_quote!(Ref<'_>);

            assert_eq!(
                spy_param_type_for_params_tuple(&ty, &lifetime_a())
                    .to_token_stream()
                    .to_string(),
                quote::quote!(Ref<'a>).to_string()
            );
        }

        #[test]
        fn test_outer_reference_is_stripped_before_substitution() {
            let ty: syn::Type = syn::parse_quote!(&'x str);

            assert_eq!(
                spy_param_type_for_params_tuple(&ty, &lifetime_a())
                    .to_token_stream()
                    .to_string(),
                quote::quote!(str).to_string()
            );
        }

        #[test]
        fn test_lifetime_nested_in_a_container_is_substituted() {
            let ty: syn::Type = syn::parse_quote!(&'x [&'x str]);

            assert_eq!(
                spy_param_type_for_params_tuple(&ty, &lifetime_a())
                    .to_token_stream()
                    .to_string(),
                quote::quote!([&'a str]).to_string()
            );
        }

        #[test]
        fn test_static_lifetime_is_kept() {
            let ty: syn::Type = syn::parse_quote!(Ref<'static>);

            assert_eq!(
                spy_param_type_for_params_tuple(&ty, &lifetime_a())
                    .to_token_stream()
                    .to_string(),
                quote::quote!(Ref<'static>).to_string()
            );
        }
    }

    mod type_needs_a_lifetime_outside_fn_sugar_tests {
        use super::*;

        #[test]
        fn test_type_without_a_lifetime_does_not_need_one() {
            let ty: syn::Type = syn::parse_quote!(String);

            assert!(!type_needs_a_lifetime_outside_fn_sugar(&ty));
        }

        #[test]
        fn test_named_lifetime_argument_needs_one() {
            let ty: syn::Type = syn::parse_quote!(Ref<'a>);

            assert!(type_needs_a_lifetime_outside_fn_sugar(&ty));
        }

        #[test]
        fn test_elided_lifetime_argument_needs_one() {
            let ty: syn::Type = syn::parse_quote!(Ref<'_>);

            assert!(type_needs_a_lifetime_outside_fn_sugar(&ty));
        }

        #[test]
        fn test_lifetime_nested_in_a_container_needs_one() {
            let ty: syn::Type = syn::parse_quote!([&'a str]);

            assert!(type_needs_a_lifetime_outside_fn_sugar(&ty));
        }

        #[test]
        fn test_static_lifetime_does_not_need_one() {
            let ty: syn::Type = syn::parse_quote!(Ref<'static>);

            assert!(!type_needs_a_lifetime_outside_fn_sugar(&ty));
        }

        #[test]
        fn test_bare_reference_without_a_lifetime_does_not_need_one() {
            let ty: syn::Type = syn::parse_quote!(&str);

            assert!(!type_needs_a_lifetime_outside_fn_sugar(&ty));
        }
    }

    mod build_reference_call_value_tests {
        use super::*;

        #[test]
        fn test_by_value_param_is_borrowed() {
            let ident: syn::Ident = syn::parse_quote!(id);
            let ty: syn::Type = syn::parse_quote!(String);

            assert_eq!(
                build_reference_call_value(&ident, &ty)
                    .to_token_stream()
                    .to_string(),
                quote::quote!(&id).to_string()
            );
        }

        #[test]
        fn test_shared_reference_param_is_forwarded_as_is() {
            let ident: syn::Ident = syn::parse_quote!(uuid);
            let ty: syn::Type = syn::parse_quote!(&str);

            assert_eq!(
                build_reference_call_value(&ident, &ty)
                    .to_token_stream()
                    .to_string(),
                quote::quote!(uuid).to_string()
            );
        }

        #[test]
        fn test_mutable_reference_param_is_reborrowed() {
            let ident: syn::Ident = syn::parse_quote!(count);
            let ty: syn::Type = syn::parse_quote!(&mut usize);

            assert_eq!(
                build_reference_call_value(&ident, &ty)
                    .to_token_stream()
                    .to_string(),
                quote::quote!(&*count).to_string()
            );
        }
    }
}
