use quote::quote;

pub fn insert_regular_regular_function_fake_call_into_fn_block(
    fn_block: &syn::Block,
    fn_fake_name: &syn::Ident,
    struct_fake_name: &syn::Ident,
    struct_name: &syn::Ident,
    fn_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>
) -> syn::Block {
    let input_idents: Vec<syn::Ident> = fn_inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .collect();

    let fake_call =
        quote! {
        #[cfg(test)]
        if #struct_fake_name::#fn_fake_name::is_set() {
            let impl_fn = #struct_fake_name::#fn_fake_name::get();
            return impl_fn(#struct_name, #(#input_idents),*);
        }
    };

    let fake_call_stmt: syn::Stmt = syn::parse(fake_call.into()).unwrap();

    let mut new_block = fn_block.clone();
    new_block.stmts.insert(0, fake_call_stmt);
    new_block
}

pub fn create_regular_regular_impl_function_fake(
    fn_name: &syn::Ident,
    fn_fake_name: &syn::Ident,
    fn_ptr_type: proc_macro2::TokenStream
) -> proc_macro2::TokenStream {
    let fn_fake_store_name = syn::Ident::new(
        &format!("{}_FAKE", fn_name.to_string().to_uppercase()),
        fn_name.span()
    );

    quote! {
        #[cfg(test)]
        thread_local! {
            static #fn_fake_store_name: std::cell::RefCell<
                fnmock::fake_store::FakeStore<#fn_ptr_type>
            > = std::cell::RefCell::new(fnmock::fake_store::FakeStore::new(stringify!(#fn_name)));
        }

        #[allow(non_camel_case_types)]
        pub(crate) struct #fn_fake_name;

        impl #fn_fake_name {
            pub(crate) fn setup(function: #fn_ptr_type) {
                #fn_fake_store_name.with_borrow_mut(|fake| {
                    fake.setup(function);
                });
            }

            pub(crate) fn clear() {
                #fn_fake_store_name.with_borrow_mut(|fake| {
                    fake.clear();
                })
            }

            pub(crate) fn is_set() -> bool {
                #fn_fake_store_name.with_borrow(|fake| { fake.is_set() })
            }

            pub(crate) fn get() -> Rc<#fn_ptr_type> {
                #fn_fake_store_name.with_borrow(|fake| { fake.get() })
            }
        }
    }
}

pub fn wrap_regular_impl_function_fakes_with_module(
    struct_fake_name: &syn::Ident,
    function_fake_blocks: Vec<proc_macro2::TokenStream>
) -> syn::ItemMod {
    let fake_module =
        quote! {
        #[cfg(test)]
        #[allow(non_snake_case)]
        pub(crate) mod #struct_fake_name {
            use std::rc::Rc;

            use super::*;

            #(#function_fake_blocks)*
        }
    };

    syn::parse(fake_module.into()).unwrap()
}
