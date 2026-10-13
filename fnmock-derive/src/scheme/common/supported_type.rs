//! The rule for which types fnmock can name in the code it generates.
//!
//! Shared by every strategy: a fake names its parameter and return types in a `Fn(..) -> ..`
//! closure trait, and a spy names its parameter types in a `Predicate<..>` bound, a `Params<'a>`
//! tuple and its matcher's own fields. A type that cannot be written in one of those positions is
//! rejected here rather than at each use site.

use syn::visit::Visit;

/// Reject types that fnmock cannot name in generated code.
///
/// `impl Trait` is rejected at any depth (`Vec<impl Trait>`, `Option<impl Trait>`), since the
/// enclosing type cannot be named either.
pub fn check_type_is_supported(ty: &syn::Type) -> syn::Result<()> {
    if let Some(impl_trait) = find_impl_trait(ty) {
        return Err(syn::Error::new_spanned(
            impl_trait,
            "`impl Trait` is not supported. Please use a generic type parameter instead.",
        ));
    }
    match ty {
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

/// The outermost `impl Trait` in `ty`, if any.
fn find_impl_trait(ty: &syn::Type) -> Option<&syn::TypeImplTrait> {
    struct Finder<'ast>(Option<&'ast syn::TypeImplTrait>);

    impl<'ast> Visit<'ast> for Finder<'ast> {
        fn visit_type_impl_trait(&mut self, impl_trait: &'ast syn::TypeImplTrait) {
            self.0.get_or_insert(impl_trait);
        }
    }

    let mut finder = Finder(None);
    finder.visit_type(ty);
    finder.0
}
