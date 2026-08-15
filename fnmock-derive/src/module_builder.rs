//! Assembly of the generated fake module from its parts.

use quote::quote;

/// Collects the pieces of a fake module and stitches them into a `syn::ItemMod`.
///
/// The pieces are set separately because the generic and non-generic cases build the store and
/// interface struct differently but wrap them identically; keeping the wrapper here means the
/// `#[cfg(test)]` gate, the `pub(crate)` visibility and the `thread_local!` block are written once
/// rather than once per case.
pub struct ModuleBuilder {
    /// The name of the generated module.
    name: Option<syn::Ident>,

    visibility: Option<syn::Visibility>,

    /// The code for the store struct, which holds the state of the fake.
    /// This is placed inside thread_local! storage in the generated module.
    store: Option<proc_macro2::TokenStream>,

    /// The code in the module beside the store.
    parts: Vec<proc_macro2::TokenStream>,
}

impl ModuleBuilder {
    /// Create a builder with no parts set yet.
    pub fn new() -> Self {
        Self {
            name: None,
            visibility: None,
            store: None,
            parts: Vec::new(),
        }
    }

    /// Set the name of the generated module.
    pub fn set_name(&mut self, name: syn::Ident) {
        self.name = Some(name);
    }

    /// Set the visibility of the generated module.
    pub fn set_visibility(&mut self, visibility: syn::Visibility) {
        self.visibility = Some(visibility);
    }

    /// Set the store declaration. It is placed inside the module's `thread_local!` block, so it
    /// should be a bare `static` declaration rather than a wrapped one.
    pub fn set_store(&mut self, store: proc_macro2::TokenStream) {
        self.store = Some(store);
    }

    /// Add a part to the module. E.g. the interface struct or spy matcher
    pub fn add_part(&mut self, part: proc_macro2::TokenStream) {
        self.parts.push(part);
    }

    /// Assemble the parts into the generated module.
    ///
    /// The module is `#[cfg(test)]`-gated and `use super::*`s its parent, so the closure trait it
    /// names can refer to types that were in scope at the faked function.
    ///
    /// # Errors
    ///
    /// Returns an error if a part was not set, or if the assembled module fails to parse. Both
    /// would be bugs in fnmock, so they are reported as such rather than as user errors.
    pub fn build_module(&self) -> syn::Result<syn::ItemMod> {
        let internal_error = |field: &str| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "internal error: the fake module's {field} was not set before building the module. This is a bug in fnmock; please report it."
                ),
            )
        };

        let name = self.name.as_ref().ok_or_else(|| internal_error("name"))?;
        let visibility = self
            .visibility
            .as_ref()
            .ok_or_else(|| internal_error("visibility"))?;
        let store = self.store.as_ref().ok_or_else(|| internal_error("store"))?;
        let parts = &self.parts;

        let code = quote! {
            #[cfg(test)]
            #visibility mod #name {
                use super::*;

                thread_local! {
                    #store
                }

                #(#parts)*
            }
        };

        syn::parse2(code).map_err(|e| {
            syn::Error::new(
                proc_macro2::Span::mixed_site(),
                format!("Failed to parse generated module code: {}", e),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_module_without_name_returns_error() {
        let mut builder = ModuleBuilder::new();
        builder.set_store(quote! {});
        builder.add_part(quote! {});

        assert!(
            builder.build_module().is_err(),
            "expected build_module to error (not panic) when the name is unset"
        );
    }

    #[test]
    fn test_build_module_without_store_returns_error() {
        let mut builder = ModuleBuilder::new();
        builder.set_name(syn::Ident::new(
            "some_module",
            proc_macro2::Span::call_site(),
        ));
        builder.add_part(quote! {});

        assert!(
            builder.build_module().is_err(),
            "expected build_module to error (not panic) when the store is unset"
        );
    }

    #[test]
    fn test_build_module_without_interface_struct_returns_error() {
        let mut builder = ModuleBuilder::new();
        builder.set_name(syn::Ident::new(
            "some_module",
            proc_macro2::Span::call_site(),
        ));
        builder.set_store(quote! {});

        assert!(
            builder.build_module().is_err(),
            "expected build_module to error (not panic) when the interface struct is unset"
        );
    }

    fn render_visibility(visibility: &syn::Visibility) -> String {
        quote! { #visibility }.to_string()
    }

    #[test]
    fn test_build_module_preserves_visibility() {
        let cases: Vec<syn::Visibility> = vec![
            syn::Visibility::Inherited,
            syn::parse_str("pub(self)").expect("valid visibility"),
            syn::parse_str("pub(super)").expect("valid visibility"),
            syn::parse_str("pub(crate)").expect("valid visibility"),
            syn::parse_str("pub(in crate::foo)").expect("valid visibility"),
            syn::parse_str("pub").expect("valid visibility"),
        ];

        for visibility in cases {
            let mut builder = ModuleBuilder::new();
            builder.set_name(syn::Ident::new(
                "some_module",
                proc_macro2::Span::call_site(),
            ));
            builder.set_visibility(visibility.clone());
            builder.set_store(quote! {});
            builder.add_part(quote! {});

            let module = builder.build_module().expect("all required parts are set");

            assert_eq!(
                render_visibility(&module.vis),
                render_visibility(&visibility),
                "expected the module's visibility to match the visibility it was built with"
            );
        }
    }
}
