use quote::quote;

pub fn insert_regular_fake_call_into_fn_block(
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

pub fn create_regular_fake_module(
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
