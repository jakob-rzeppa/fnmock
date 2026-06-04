use quote::quote;

use crate::fakeable::{
    function::{ generic::generic_fakeable_function, regular::regular_fakeable_function },
    generic_helpers::{ build_function_ptr_type, generate_function_fake_name },
    helpers::{ extract_param_idents, extract_param_types },
};

mod regular;
mod generic;

pub fn fakeable_function(item_fn: syn::ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    // --- Names ---
    let fn_name = &item_fn.sig.ident;
    let fn_fake_name = generate_function_fake_name(fn_name);

    // --- Params ---
    let fn_inputs = &item_fn.sig.inputs.iter().cloned().collect::<Vec<_>>();
    let param_idents = extract_param_idents(fn_inputs);
    let param_types = extract_param_types(fn_inputs);

    // --- Build function pointer type ---
    let fn_output = &item_fn.sig.output;
    let fn_ptr_type = build_function_ptr_type(&param_types, fn_output);

    // --- Function block ---
    let fn_block = &item_fn.block;

    // --- Handle the function based on whether it has generics ---
    let fn_generics = &item_fn.sig.generics;
    let (new_fn_block, fake_module) = if fn_generics.params.is_empty() {
        regular_fakeable_function(fn_name, &fn_fake_name, &param_idents, fn_ptr_type, fn_block)?
    } else {
        generic_fakeable_function(
            fn_name,
            &fn_fake_name,
            &param_idents,
            fn_ptr_type,
            fn_block,
            fn_generics
        )?
    };

    // --- Build the function with the new block ---
    let mut new_item_fn = item_fn.clone();
    new_item_fn.block = Box::new(new_fn_block);

    // --- Combine the function and the fake module ---
    let expanded = quote! {
        #new_item_fn

        #fake_module
    };

    Ok(expanded.into())
}
