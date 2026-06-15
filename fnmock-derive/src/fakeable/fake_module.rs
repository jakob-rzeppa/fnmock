use quote::quote;

use crate::fakeable::extract::info::FakeableInfo;

pub fn generate_fake_module_code(info: &FakeableInfo) -> syn::Result<syn::ItemMod> {
    if let Some(_) = &info.generic_info {
        generate_generic_fake_module_code(info)
    } else {
        generate_regular_fake_module_code(info)
    }
}

fn generate_regular_fake_module_code(info: &FakeableInfo) -> syn::Result<syn::ItemMod> {
    let module_name = &info.module_name;
    let store_name = &info.store_name;
    let display_name = &info.display_name;
    let interface_struct_name = &info.interface_struct_name;
    let fn_ptr_type = &info.fn_ptr_type;

    let code =
        quote! {
        mod #module_name {
            use super::*;

            thread_local! {
                static #store_name: std::cell::RefCell<fnmock::fake_store::FakeStore<#fn_ptr_type>> =
                    std::cell::RefCell::new(fnmock::fake_store::FakeStore::new(stringify!(#display_name)));
            }

            pub(crate) struct #interface_struct_name;

            impl #interface_struct_name {
                pub(crate) fn new() -> Self {
                    Self
                }

                pub(crate) fn setup(self, function: #fn_ptr_type) -> Self {
                    #store_name.with(|store| {
                        store.borrow_mut().setup(function);
                    });
                    self
                }

                pub(crate) fn clear(self) -> Self {
                    #store_name.with(|store| {
                        store.borrow_mut().clear();
                    });
                    self
                }

                pub(crate) fn is_set(&self) -> bool {
                    #store_name.with(|store| store.borrow().is_set())
                }

                pub(crate) fn get(&self) -> std::rc::Rc<#fn_ptr_type> {
                    #store_name.with(|store| store.borrow().get())
                }
            }
        }
    };

    syn::parse2(code)
}

fn generate_generic_fake_module_code(info: &FakeableInfo) -> syn::Result<syn::ItemMod> {
    let module_name = &info.module_name;
    let store_name = &info.store_name;
    let display_name = &info.display_name;
    let interface_struct_name = &info.interface_struct_name;
    let fn_ptr_type = &info.fn_ptr_type;

    let (generic_count, generic_idents, generic_params, generic_type_ids) = if
        let Some(generic_info) = &info.generic_info
    {
        (
            generic_info.generic_count,
            &generic_info.generic_idents,
            &generic_info.generic_params,
            &generic_info.generic_type_ids,
        )
    } else {
        unreachable!(
            "generate_generic_fake_module_code should only be called when info.generic_info is Some"
        );
    };

    let code =
        quote! {
        mod #module_name {
            use super::*;

        thread_local! {
            static #store_name: std::cell::RefCell<fnmock::generic_fake_store::GenericFakeStore<#generic_count>> = 
                std::cell::RefCell::new(
                    fnmock::generic_fake_store::GenericFakeStore::new(stringify!(#display_name))
                );
        }

        pub(crate) struct #interface_struct_name<#(#generic_params),*> {
            _marker: std::marker::PhantomData<(#(#generic_idents),*)>,
        }

        impl<#(#generic_params),*> #interface_struct_name<#(#generic_idents),*> {
            pub(crate) fn new() -> Self {
                Self {
                    _marker: std::marker::PhantomData,
                }
            }

            pub(crate) fn setup(self, function: #fn_ptr_type) -> Self {
                #store_name.with_borrow_mut(|fake| {
                    fake.setup_for([#(#generic_type_ids),*], function);
                });
                self
            }

            pub(crate) fn clear(self) -> Self {
                #store_name.with_borrow_mut(|fake| {
                    fake.clear_for([#(#generic_type_ids),*]);
                });
                self
            }

            pub(crate) fn is_set(&self) -> bool {
                #store_name.with_borrow(|fake| {
                    fake.is_set_for([#(#generic_type_ids),*])
                })
            }

            pub(crate) fn get(&self) -> std::rc::Rc<#fn_ptr_type> {
                #store_name.with_borrow(|fake| {
                    fake.get_for::<#fn_ptr_type>([#(#generic_type_ids),*])
                })
            }
        }
        }
    };

    syn::parse2(code)
}
