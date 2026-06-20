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
