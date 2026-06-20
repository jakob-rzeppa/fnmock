use quote::quote;

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
    pub fn new() -> Self {
        Self {
            name: None,
            store: None,
            interface_struct: None,
        }
    }

    pub fn set_name(&mut self, name: syn::Ident) {
        self.name = Some(name);
    }

    pub fn set_store(&mut self, store: proc_macro2::TokenStream) {
        self.store = Some(store);
    }

    pub fn set_interface_struct(&mut self, interface_struct: proc_macro2::TokenStream) {
        self.interface_struct = Some(interface_struct);
    }

    pub fn build_module(&self) -> syn::Result<syn::ItemMod> {
        let name = self.name.as_ref().expect("Module name must be defined to build Module.");
        let store = self.store.as_ref().expect("Store must be defined to build Module.");
        let interface_struct = self.interface_struct
            .as_ref()
            .expect("Interface struct must be defined to build Module.");

        let code =
            quote! {
            #[cfg(test)]
            mod #name {
                use super::*;

                thread_local! {
                    #store
                }

                #interface_struct
            }
        };

        syn::parse2(code)
    }
}
