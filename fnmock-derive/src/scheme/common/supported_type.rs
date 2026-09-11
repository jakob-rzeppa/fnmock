//! The rule for which types fnmock can name in the code it generates.
//!
//! Shared by every strategy: a fake names its parameter and return types in a `Fn(..) -> ..`
//! closure trait, and a spy names its parameter types in a `Predicate<..>` bound, a `Params<'a>`
//! tuple and its matcher's own fields. A type that cannot be written in one of those positions is
//! rejected here rather than at each use site.

/// Reject types that fnmock cannot name in generated code.
pub fn check_type_is_supported(ty: &syn::Type) -> syn::Result<()> {
    match ty {
        syn::Type::ImplTrait(_) => Err(syn::Error::new_spanned(
            ty,
            "`impl Trait` is not supported. Please use a generic type parameter instead.",
        )),
        syn::Type::Infer(_) => Err(syn::Error::new_spanned(
            ty,
            "The inferred type `_` is not supported. Please specify the type explicitly.",
        )),
        syn::Type::Macro(_) => Err(syn::Error::new_spanned(
            ty,
            "Macros in the function signiture are not supported.",
        )),
        syn::Type::Never(_) => Err(syn::Error::new_spanned(
            ty,
            "The never type `!` is not supported.",
        )),
        _ => Ok(()),
    }
}
