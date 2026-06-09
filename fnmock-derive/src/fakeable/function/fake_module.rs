use quote::quote;

use crate::fakeable::function::{ info::FakeableFnInfo };

pub fn build_fake_module(info: &FakeableFnInfo) -> syn::ItemMod {
    let fake_module_name = &info.fake_module_name;

    let store = build_store(info);

    let fake_interface_struct = build_fake_interface_struct(info);

    syn::parse2(
        quote! {
        #[cfg(test)]
        pub(crate) mod #fake_module_name {
            use super::*;

            #store

            #fake_interface_struct
        }
    }
    ).expect("Failed to parse generated fake module")
}

fn build_store(info: &FakeableFnInfo) -> proc_macro2::TokenStream {
    let fn_name = &info.fn_name;
    let fake_store_name = &info.fake_store_name;

    if let Some(generic_info) = &info.generic_info {
        let num_of_generics = generic_info.fn_generic_params.len();
        quote! {
            thread_local! {
                static #fake_store_name: std::cell::RefCell<fnmock::generic_fake_store::GenericFakeStore<#num_of_generics>> = std::cell::RefCell::new(
                    fnmock::generic_fake_store::GenericFakeStore::new(stringify!(#fn_name))
                );
            }
        }
    } else {
        let fn_ptr_type = &info.fn_ptr_type;

        quote! {
            thread_local! {
                static #fake_store_name: std::cell::RefCell<fnmock::fake_store::FakeStore<#fn_ptr_type>> = std::cell::RefCell::new(
                    fnmock::fake_store::FakeStore::new(stringify!(#fn_name))
                );
            }
        }
    }
}

fn build_fake_interface_struct(info: &FakeableFnInfo) -> proc_macro2::TokenStream {
    let fake_api_struct_name = &info.fake_api_struct_name;
    let fn_ptr_type = &info.fn_ptr_type;
    let fake_store_name = &info.fake_store_name;

    if let Some(generic_info) = &info.generic_info {
        let fn_generic_params = &generic_info.fn_generic_params;
        let fn_generic_idents = &generic_info.fn_generic_idents;
        let fn_generic_type_ids = &generic_info.fn_generic_type_ids;

        quote! {
            pub(crate) struct #fake_api_struct_name<#(#fn_generic_params),*> {
                _marker: std::marker::PhantomData<(#(#fn_generic_idents),*)>,
            }

            impl<#(#fn_generic_params),*> #fake_api_struct_name<#(#fn_generic_idents),*> {
                pub(crate) fn new() -> Self {
                    Self { _marker: std::marker::PhantomData }
                }

                pub(crate) fn setup(&self, function: #fn_ptr_type) {
                    #fake_store_name.with_borrow_mut(|fake| {
                        fake.setup_for([#(#fn_generic_type_ids),*], function);
                    });
                }

                pub(crate) fn clear(&self) {
                    #fake_store_name.with_borrow_mut(|fake| {
                        fake.clear_for([#(#fn_generic_type_ids),*]);
                    })
                }

                pub(crate) fn is_set(&self) -> bool {
                    #fake_store_name.with_borrow(|fake| { 
                        fake.is_set_for([#(#fn_generic_type_ids),*])
                    })
                }

                pub(crate) fn get(&self) -> std::rc::Rc<#fn_ptr_type> {
                    #fake_store_name.with_borrow(|fake| { 
                        fake.get_for::<#fn_ptr_type>([#(#fn_generic_type_ids),*]) 
                    })
                }
            }
        }
    } else {
        quote! {
            pub(crate) struct #fake_api_struct_name;

            impl #fake_api_struct_name {
                pub(crate) fn new() -> Self {
                    Self
                }

                pub(crate) fn setup(&self, function: #fn_ptr_type) {
                    #fake_store_name.with_borrow_mut(|fake| {
                        fake.setup(function);
                    });
                }

                pub(crate) fn clear(&self) {
                    #fake_store_name.with_borrow_mut(|fake| {
                        fake.clear();
                    })
                }

                pub(crate) fn is_set(&self) -> bool {
                    #fake_store_name.with_borrow(|fake| { fake.is_set() })
                }

                pub(crate) fn get(&self) -> std::rc::Rc<#fn_ptr_type> {
                    #fake_store_name.with_borrow(|fake| { fake.get() })
                }
            }
        }
    }
}
