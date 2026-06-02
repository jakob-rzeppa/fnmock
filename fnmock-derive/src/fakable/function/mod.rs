use proc_macro::TokenStream;
use quote::quote;

use crate::fakable::function::{
    generic::{ create_generic_fake_module, insert_generic_fake_call_into_fn_block },
    regular::{ create_regular_fake_module, insert_regular_fake_call_into_fn_block },
};

mod regular;
mod generic;

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

    let (new_fn_block, fake_module) = if fn_generics.params.is_empty() {
        let new_fn_block = insert_regular_fake_call_into_fn_block(
            fn_block,
            &fn_fake_name,
            fn_inputs
        );
        let fake_module = create_regular_fake_module(fn_name, &fn_fake_name, fn_ptr_type);
        (new_fn_block, fake_module)
    } else {
        let new_fn_block = insert_generic_fake_call_into_fn_block(
            fn_block,
            &fn_fake_name,
            fn_inputs,
            fn_generics
        );
        let fake_module = create_generic_fake_module(fn_name, &fn_fake_name, fn_generics);
        (new_fn_block, fake_module)
    };

    let mut new_item_fn = item_fn.clone();
    new_item_fn.block = Box::new(new_fn_block);

    let expanded = quote! {
        #new_item_fn

        #fake_module
    };

    Ok(expanded.into())
}
