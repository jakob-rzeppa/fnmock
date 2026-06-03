use quote::quote;

pub fn regular_fakeable_function(
    fn_name: &syn::Ident,
    fn_fake_name: &syn::Ident,
    input_idents: &[syn::Ident],
    fn_ptr_type: proc_macro2::TokenStream,
    fn_block: &syn::Block
) -> syn::Result<(syn::Block, syn::ItemMod)> {
    let fake_module = create_regular_fake_module(fn_name, fn_fake_name, fn_ptr_type);
    let new_fn_block = insert_regular_fake_call_into_fn_block(fn_block, fn_fake_name, input_idents);

    Ok((new_fn_block, fake_module))
}

/// Insert a call to the fake implementation at the start of the function block for a regular (non-generic) function.
///
/// # Params
///
/// - `fn_block`: The original function block to modify.
/// - `fn_fake_name`: The identifier for the fake struct that holds the fake implementation (e.g. `get_user_fake`).
/// - `input_idents`: The identifiers of the function's input parameters (e.g. `id`, `name`).
fn insert_regular_fake_call_into_fn_block(
    fn_block: &syn::Block,
    fn_fake_name: &syn::Ident,
    input_idents: &[syn::Ident]
) -> syn::Block {
    let fake_call =
        quote! {
            #[cfg(test)]
            if #fn_fake_name::is_set() {
                let fake_implementation = #fn_fake_name::get();
                return fake_implementation(#(#input_idents),*);
            }
        };

    let fake_call_stmt: syn::Stmt = syn
        ::parse(fake_call.into())
        .expect("Failed to parse generated fake call");

    let mut new_block = fn_block.clone();
    new_block.stmts.insert(0, fake_call_stmt);
    new_block
}

/// Create a fake module for a regular (non-generic) function.
///
/// # Params
///
/// - `fn_name`: The name of the original function being faked (used for naming the fake module).
/// - `fn_fake_name`: The identifier for the fake struct that will hold the fake implementation (e.g. `get_user_fake`).
/// - `fn_ptr_type`: The function pointer type of the original function (e.g. `fn(i32) -> String`).
fn create_regular_fake_module(
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

    syn::parse(fake_module.into()).expect("Failed to parse generated fake module")
}
