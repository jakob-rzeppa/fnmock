use proc_macro::TokenStream;
use quote::quote;

pub fn fakable_function(item_fn: syn::ItemFn) -> syn::Result<TokenStream> {
    let fn_name = &item_fn.sig.ident;
    let fn_generics = &item_fn.sig.generics;
    let fn_inputs = &item_fn.sig.inputs;
    let fn_output = &item_fn.sig.output;
    let fn_block = &item_fn.block;

    let fn_fake_name = syn::Ident::new(&format!("{}_fake", fn_name), fn_name.span());

    // Extract the input types to construct the function pointer type
    let mut input_types = Vec::new();
    for arg in fn_inputs.iter() {
        match arg {
            syn::FnArg::Typed(pat_type) => {
                input_types.push(&pat_type.ty);
            }
            syn::FnArg::Receiver(_) => {
                return Err(
                    syn::Error::new_spanned(
                        arg,
                        "self parameters are not supported in a #[fakeable] function, use #[fakeable] on an impl block instead."
                    )
                );
            }
        }
    }

    let fn_ptr_type = quote! {
        fn(#(#input_types),*) #fn_output
    };

    let new_fn_block = insert_fake_call_into_fn_block(fn_block, &fn_fake_name, fn_inputs);
    let fake_module = create_fake_module(fn_name, &fn_fake_name, fn_ptr_type);

    let mut new_item_fn = item_fn.clone();
    new_item_fn.block = Box::new(new_fn_block);

    let expanded = quote! {
        #new_item_fn

        #fake_module
    };

    Ok(expanded.into())
}

fn insert_fake_call_into_fn_block(
    fn_block: &syn::Block,
    fn_fake_name: &syn::Ident,
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
        if #fn_fake_name::is_set() {
            let impl_fn = #fn_fake_name::get();
            return impl_fn(#(#input_idents),*);
        }
    };

    let fake_call_stmt: syn::Stmt = syn::parse(fake_call.into()).unwrap();

    let mut new_block = fn_block.clone();
    new_block.stmts.insert(0, fake_call_stmt);
    new_block
}

fn create_fake_module(
    fn_name: &syn::Ident,
    fn_fake_name: &syn::Ident,
    fn_ptr_type: proc_macro2::TokenStream
) -> syn::ItemMod {
    let fake_module =
        quote! {
        #[cfg(test)]
        pub(crate) mod #fn_fake_name {
            use std::rc::Rc;

            use fnmock::{ fake_store::FakeStore };

            thread_local! {
                static FAKE: std::cell::RefCell<FakeStore<#fn_ptr_type>> = std::cell::RefCell::new(
                    FakeStore::new(stringify!(#fn_name))
                );
            }

            pub(crate) fn setup(function: #fn_ptr_type) {
                FAKE.with_borrow_mut(|fake| {
                    fake.setup(function);
                });
            }

            pub(crate) fn clear() {
                FAKE.with_borrow_mut(|fake| {
                    fake.clear();
                })
            }

            pub(crate) fn is_set() -> bool {
                FAKE.with_borrow(|fake| { fake.is_set() })
            }

            pub(crate) fn get() -> Rc<#fn_ptr_type> {
                FAKE.with_borrow(|fake| { fake.get() })
            }
        }
    };

    syn::parse(fake_module.into()).unwrap()
}
