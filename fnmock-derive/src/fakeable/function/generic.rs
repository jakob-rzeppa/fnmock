use quote::quote;

use crate::fakeable::{
    function::generic,
    generic_helpers::{ build_type_id_array, extract_generic_idents, extract_generic_params },
};

/// Handle a generic function annotated with #[fakeable].
///
/// This function generates the necessary fake module and modifies the original function block to include a call to the fake implementation if it is set up.
///
/// # Params
///
/// - `fn_name`: The identifier of the original function being faked (e.g. `get_user`).
/// - `fn_fake_name`: The identifier for the fake struct that will hold the fake implementation (e.g. `get_user_fake`).
/// - `input_idents`: The identifiers of the function's input parameters (e.g. `id`, `name`).
/// - `fn_ptr_type`: The function pointer type of the original function (e.g. `fn(T, i32) -> String`).
/// - `fn_block`: The original function block to modify.
/// - `fn_generics`: The generic parameters of the original function (e.g. `<T, U: Display>`).
///
/// # Returns
///
/// A tuple containing the modified function block and the generated fake module.
pub fn generic_fakeable_function(
    fn_name: &syn::Ident,
    fn_fake_name: &syn::Ident,
    input_idents: &[syn::Ident],
    fn_ptr_type: proc_macro2::TokenStream,
    fn_block: &syn::Block,
    fn_generics: &syn::Generics
) -> syn::Result<(syn::Block, syn::ItemMod)> {
    let generic_params = extract_generic_params(fn_generics);
    let generic_idents = extract_generic_idents(&generic_params);
    let generic_type_id_array = build_type_id_array(&generic_idents);

    let fake_module = generic::create_generic_fake_module(
        fn_name,
        fn_fake_name,
        fn_ptr_type,
        &generic_params,
        &generic_type_id_array
    )?;

    let new_fn_block = generic::insert_generic_fake_call_into_fn_block(
        fn_block,
        fn_fake_name,
        input_idents,
        &generic_idents
    );

    Ok((new_fn_block, fake_module))
}

/// Insert a fake call into the function block for a generic function.
///
/// # Params
///
/// - `fn_block`: The original function block to modify.
/// - `fn_fake_name`: The identifier for the fake struct that holds the fake implementation.
/// - `input_idents`: The identifiers of the function's input parameters (e.g. `id`, `name`).
/// - `generic_idents`: The identifiers of the function's generic type parameters (e.g. `T`, `U`).
fn insert_generic_fake_call_into_fn_block(
    fn_block: &syn::Block,
    fn_fake_name: &syn::Ident,
    input_idents: &[syn::Ident],
    generic_idents: &[syn::Ident]
) -> syn::Block {
    let fake_call =
        quote! {
            #[cfg(test)]
            if #fn_fake_name::is_set_for::<#(#generic_idents),*>() {
                let fake_implementation = #fn_fake_name::get_for::<#(#generic_idents),*>();
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

/// Create a fake module for a generic function.
///
/// # Params
///
/// - `fn_name`: The name of the original function being faked (used for naming the fake module).
/// - `fn_fake_name`: The identifier for the fake struct that will hold the fake implementation (e.g. `get_user_fake`).
/// - `fn_ptr_type`: The function pointer type of the original function (e.g. `fn(T, i32) -> String`).
/// - `generic_params`: The generic parameters of the original function (e.g. `<T, U: Display>`).
/// - `generic_type_id_array`: An array of `TypeId` expressions corresponding to the generic parameters (e.g. `[TypeId::of::<T>(), TypeId::of::<U>()]`).
fn create_generic_fake_module(
    fn_name: &syn::Ident,
    fn_fake_name: &syn::Ident,
    fn_ptr_type: proc_macro2::TokenStream,
    generic_params: &[syn::TypeParam],
    generic_type_id_array: &[proc_macro2::TokenStream]
) -> syn::Result<syn::ItemMod> {
    let generic_count = generic_params.len();

    let fake_module =
        quote! {
        #[cfg(test)]
        pub(crate) mod #fn_fake_name {
            use fnmock::generic_fake_store::GenericFakeStore;

            use super::*;

            thread_local! {
                static FAKE: std::cell::RefCell<GenericFakeStore<#generic_count>> = std::cell::RefCell::new(
                    GenericFakeStore::new(stringify!(#fn_name))
                );
            }

            pub(crate) fn setup<#(#generic_params),*>(function: #fn_ptr_type) {
                FAKE.with_borrow_mut(|fake| {
                    fake.setup_for([#(#generic_type_id_array),*], function);
                });
            }

            pub(crate) fn clear() {
                FAKE.with_borrow_mut(|fake| {
                    fake.clear();
                })
            }

            pub(crate) fn clear_for<#(#generic_params),*>() {
                FAKE.with_borrow_mut(|fake| {
                    fake.clear_for([#(#generic_type_id_array),*]);
                })
            }

            pub(crate) fn is_set_for<#(#generic_params),*>() -> bool {
                FAKE.with_borrow(|fake| { fake.is_set_for([#(#generic_type_id_array),*]) })
            }

            pub(crate) fn get_for<#(#generic_params),*>() -> std::rc::Rc<#fn_ptr_type> {
                FAKE.with_borrow(|fake| { fake.get_for::<#fn_ptr_type>([#(#generic_type_id_array),*]) })
            }
        }
    };

    Ok(syn::parse(fake_module.into()).expect("Failed to parse generated fake module"))
}
