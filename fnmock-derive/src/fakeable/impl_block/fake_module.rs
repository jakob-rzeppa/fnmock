use crate::fakeable::impl_block::info::FakeableImplFnInfo;
use quote::quote;

pub fn build_fake_module(info: &[FakeableImplFnInfo]) -> syn::Result<syn::ItemMod> {
    if info.is_empty() {
        return Err(
            syn::Error::new(proc_macro2::Span::call_site(), "No functions to generate fakes for")
        );
    }

    let fake_module = &info[0].fake_module;

    // Generate fake implementations for each function
    let mut function_fakes = Vec::new();
    for func_info in info {
        let fake_impl = generate_function_fake(func_info)?;
        function_fakes.push(fake_impl);
    }

    // Build the complete module
    let module_code =
        quote! {
        #[cfg(test)]
        pub(crate) mod #fake_module {
            use super::*;

            #(#function_fakes)*
        }
    };

    syn::parse(module_code.into()).map_err(|_| {
        syn::Error::new(proc_macro2::Span::call_site(), "Failed to parse generated fake module")
    })
}

fn generate_function_fake(info: &FakeableImplFnInfo) -> syn::Result<proc_macro2::TokenStream> {
    let fake_store_name = &info.fake_store_name;
    let fake_api_name = &info.fake_api_name;
    let fn_name = &info.fn_name;
    let fn_ptr_type = &info.fn_ptr_type;
    let fn_param_idents = &info.fn_param_idents;

    // Determine if we need generics
    let has_generics = info.generic_info.is_some();

    if has_generics {
        generate_generic_function_fake(
            info,
            fake_store_name,
            fake_api_name,
            fn_name,
            fn_ptr_type,
            fn_param_idents
        )
    } else {
        generate_regular_function_fake(fake_store_name, fake_api_name, fn_name, fn_ptr_type)
    }
}

fn generate_regular_function_fake(
    fake_store_name: &syn::Ident,
    fake_api_name: &syn::Ident,
    fn_name: &syn::Ident,
    fn_ptr_type: &syn::Type
) -> syn::Result<proc_macro2::TokenStream> {
    let code =
        quote! {
        thread_local! {
            static #fake_store_name: std::cell::RefCell<fnmock::fake_store::FakeStore<#fn_ptr_type>> = 
                std::cell::RefCell::new(
                    fnmock::fake_store::FakeStore::new(stringify!(#fn_name))
                );
        }

        pub(crate) struct #fake_api_name;

        impl #fake_api_name {
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
                });
            }

            pub(crate) fn is_set(&self) -> bool {
                #fake_store_name.with_borrow(|fake| {
                    fake.is_set()
                })
            }

            pub(crate) fn get(&self) -> std::rc::Rc<#fn_ptr_type> {
                #fake_store_name.with_borrow(|fake| {
                    fake.get()
                })
            }
        }
    };

    Ok(code)
}

fn generate_generic_function_fake(
    info: &FakeableImplFnInfo,
    fake_store_name: &syn::Ident,
    fake_api_name: &syn::Ident,
    fn_name: &syn::Ident,
    fn_ptr_type: &syn::Type,
    _fn_param_idents: &[syn::Ident]
) -> syn::Result<proc_macro2::TokenStream> {
    let generic_info = info.generic_info
        .as_ref()
        .ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected generic info for generic function fake"
            )
        })?;

    let struct_generic_params = &generic_info.struct_generic_params;
    let fn_generic_params = &generic_info.fn_generic_params;

    let all_generic_params = if struct_generic_params.is_empty() {
        fn_generic_params.clone()
    } else if fn_generic_params.is_empty() {
        struct_generic_params.clone()
    } else {
        let mut combined = struct_generic_params.clone();
        combined.extend(fn_generic_params.clone());
        combined
    };

    let struct_generic_idents = &generic_info.struct_generic_idents;
    let fn_generic_idents = &generic_info.fn_generic_idents;

    let all_generic_idents = if struct_generic_idents.is_empty() {
        fn_generic_idents.clone()
    } else if fn_generic_idents.is_empty() {
        struct_generic_idents.clone()
    } else {
        let mut combined = struct_generic_idents.clone();
        combined.extend(fn_generic_idents.clone());
        combined
    };

    let struct_type_ids = &generic_info.struct_generic_type_ids;
    let fn_type_ids = &generic_info.fn_generic_type_ids;

    let all_type_ids = {
        let mut combined = struct_type_ids.clone();
        combined.extend(fn_type_ids.clone());
        combined
    };

    let total_generics = all_type_ids.len();

    let phantom_tuple = if all_generic_idents.is_empty() {
        quote! { () }
    } else {
        quote! { (#(#all_generic_idents),*) }
    };

    let code =
        quote! {
        thread_local! {
            static #fake_store_name: std::cell::RefCell<fnmock::generic_fake_store::GenericFakeStore<#total_generics>> = 
                std::cell::RefCell::new(
                    fnmock::generic_fake_store::GenericFakeStore::new(stringify!(#fn_name))
                );
        }

        pub(crate) struct #fake_api_name<#(#all_generic_params),*> {
            _marker: std::marker::PhantomData<#phantom_tuple>,
        }

        impl<#(#all_generic_params),*> #fake_api_name<#(#all_generic_idents),*> {
            pub(crate) fn new() -> Self {
                Self {
                    _marker: std::marker::PhantomData,
                }
            }

            pub(crate) fn setup(&self, function: #fn_ptr_type) {
                #fake_store_name.with_borrow_mut(|fake| {
                    fake.setup_for([#(#all_type_ids),*], function);
                });
            }

            pub(crate) fn clear(&self) {
                #fake_store_name.with_borrow_mut(|fake| {
                    fake.clear_for([#(#all_type_ids),*]);
                });
            }

            pub(crate) fn is_set(&self) -> bool {
                #fake_store_name.with_borrow(|fake| {
                    fake.is_set_for([#(#all_type_ids),*])
                })
            }

            pub(crate) fn get(&self) -> std::rc::Rc<#fn_ptr_type> {
                #fake_store_name.with_borrow(|fake| {
                    fake.get_for::<#fn_ptr_type>([#(#all_type_ids),*])
                })
            }
        }
    };

    Ok(code)
}
