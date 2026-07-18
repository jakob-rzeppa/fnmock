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

    /// The code for the store struct, which holds the state of the fake.
    /// This is placed inside thread_local! storage in the generated module.
    store: Option<proc_macro2::TokenStream>,

    /// The code for the interface struct, which provides methods to interact with the fake.
    ///
    /// This must include the impl block for the interface struct, which defines the methods that users will call to set up and verify the fake.
    interface_struct: Option<proc_macro2::TokenStream>,
}

impl ModuleBuilder {
    /// Create a builder with no parts set yet.
    pub fn new() -> Self {
        Self {
            name: None,
            store: None,
            interface_struct: None,
        }
    }

    /// Set the name of the generated module.
    pub fn set_name(&mut self, name: syn::Ident) {
        self.name = Some(name);
    }

    /// Set the store declaration. It is placed inside the module's `thread_local!` block, so it
    /// should be a bare `static` declaration rather than a wrapped one.
    pub fn set_store(&mut self, store: proc_macro2::TokenStream) {
        self.store = Some(store);
    }

    /// Set the interface struct, including its impl block — see the field's own note.
    pub fn set_interface_struct(&mut self, interface_struct: proc_macro2::TokenStream) {
        self.interface_struct = Some(interface_struct);
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
                )
            )
        };

        let name = self.name.as_ref().ok_or_else(|| internal_error("name"))?;
        let store = self.store.as_ref().ok_or_else(|| internal_error("store"))?;
        let interface_struct = self
            .interface_struct
            .as_ref()
            .ok_or_else(|| internal_error("interface struct"))?;

        let code = quote! {
            #[cfg(test)]
            pub(crate) mod #name {
                use super::*;

                thread_local! {
                    #store
                }

                #interface_struct
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
        builder.set_interface_struct(quote! {});

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
        builder.set_interface_struct(quote! {});

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
}
