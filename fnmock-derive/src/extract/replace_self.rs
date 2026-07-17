/// Helper struct to replace `Self` types in a return type with the actual type of `Self` from the impl block.
pub struct ReplaceSelf<'a> {
    self_ty: &'a syn::Type,
}

impl<'a> ReplaceSelf<'a> {
    pub fn new(self_ty: &'a syn::Type) -> Self {
        ReplaceSelf { self_ty }
    }
}

/// Implements a visitor that traverses a `syn::Type` and replaces any occurrences of `Self` with the provided type from the impl block.
impl syn::visit_mut::VisitMut for ReplaceSelf<'_> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let syn::Type::Path(syn::TypePath { qself: None, path }) = ty {
            if path.is_ident("Self") {
                *ty = self.self_ty.clone();
                return;
            }
        }

        syn::visit_mut::visit_type_mut(self, ty);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use syn::visit_mut::VisitMut;

    /// Runs `ReplaceSelf` over `input`, substituting `self_ty` for bare `Self`, and returns the
    /// normalized token string of the result.
    fn replace(input: syn::Type, self_ty: &syn::Type) -> String {
        let mut ty = input;
        let mut replacer = ReplaceSelf::new(self_ty);
        replacer.visit_type_mut(&mut ty);
        ty.to_token_stream().to_string()
    }

    #[test]
    fn test_bare_self_is_replaced() {
        let self_ty: syn::Type = syn::parse_quote!(MyStruct);
        let ty: syn::Type = syn::parse_quote!(Self);

        assert_eq!(replace(ty, &self_ty), quote::quote!(MyStruct).to_string());
    }

    #[test]
    fn test_reference_to_self_is_replaced() {
        let self_ty: syn::Type = syn::parse_quote!(MyStruct);
        let ty: syn::Type = syn::parse_quote!(&Self);

        assert_eq!(replace(ty, &self_ty), quote::quote!(&MyStruct).to_string());
    }

    #[test]
    fn test_generic_containing_self_is_replaced() {
        let self_ty: syn::Type = syn::parse_quote!(MyStruct);
        let ty: syn::Type = syn::parse_quote!(Vec<Self>);

        assert_eq!(
            replace(ty, &self_ty),
            quote::quote!(Vec<MyStruct>).to_string()
        );
    }

    #[test]
    fn test_deeply_nested_self_is_replaced() {
        let self_ty: syn::Type = syn::parse_quote!(MyStruct);
        let ty: syn::Type = syn::parse_quote!(Option<Box<Self>>);

        // Note: compared against a re-emitted `syn::Type` (rather than a literal `quote!`
        // token stream) because syn splits a nested `>>` into two separate `>` tokens when
        // printing a parsed AST, which does not textually match the single `>>` token that
        // `quote!` would emit from source text. Round-tripping the expected value through
        // `syn::parse_quote!` keeps both sides on the same normalization.
        let expected: syn::Type = syn::parse_quote!(Option<Box<MyStruct>>);
        assert_eq!(
            replace(ty, &self_ty),
            expected.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_self_in_tuple_is_replaced() {
        let self_ty: syn::Type = syn::parse_quote!(MyStruct);
        let ty: syn::Type = syn::parse_quote!((Self, i32));

        assert_eq!(
            replace(ty, &self_ty),
            quote::quote!((MyStruct, i32)).to_string()
        );
    }

    #[test]
    fn test_type_without_self_is_unchanged() {
        let self_ty: syn::Type = syn::parse_quote!(MyStruct);
        let ty: syn::Type = syn::parse_quote!(Vec<i32>);

        assert_eq!(replace(ty, &self_ty), quote::quote!(Vec<i32>).to_string());
    }

    #[test]
    fn test_lookalike_identifier_is_not_replaced() {
        let self_ty: syn::Type = syn::parse_quote!(MyStruct);
        let ty: syn::Type = syn::parse_quote!(SelfLike);

        assert_eq!(replace(ty, &self_ty), quote::quote!(SelfLike).to_string());
    }

    #[test]
    fn test_bare_self_replaced_with_generic_target() {
        let self_ty: syn::Type = syn::parse_quote!(Foo<T>);
        let ty: syn::Type = syn::parse_quote!(Self);

        assert_eq!(replace(ty, &self_ty), quote::quote!(Foo<T>).to_string());
    }

    #[test]
    fn test_nested_self_replaced_with_generic_target() {
        let self_ty: syn::Type = syn::parse_quote!(Foo<T>);
        let ty: syn::Type = syn::parse_quote!(Vec<Self>);

        // See the note in `test_deeply_nested_self_is_replaced`: the nested `>>` requires
        // comparing against a re-emitted `syn::Type` rather than a literal `quote!` stream.
        let expected: syn::Type = syn::parse_quote!(Vec<Foo<T>>);
        assert_eq!(
            replace(ty, &self_ty),
            expected.to_token_stream().to_string()
        );
    }

    /// Characterization test: the visitor recurses into the qself of a qualified path
    /// (`<Self as SomeTrait>::Item`), so the inner `Self` is replaced even though the
    /// overall type is not a bare `Self` path.
    #[test]
    fn test_self_in_qualified_path_qself_is_replaced() {
        let self_ty: syn::Type = syn::parse_quote!(MyStruct);
        let ty: syn::Type = syn::parse_quote!(<Self as SomeTrait>::Item);

        assert_eq!(
            replace(ty, &self_ty),
            quote::quote!(<MyStruct as SomeTrait>::Item).to_string()
        );
    }
}
