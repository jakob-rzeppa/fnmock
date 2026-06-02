use proc_macro::TokenStream;
use quote::quote;

use crate::fakeable::impl_block::regular::{
    create_regular_regular_impl_function_fake,
    insert_regular_regular_function_fake_call_into_fn_block,
    wrap_regular_impl_function_fakes_with_module,
};

mod regular;
mod generic;

pub fn fakable_impl_block(mut item_impl: syn::ItemImpl) -> syn::Result<TokenStream> {
    // Get the struct name being implemented
    let struct_name = match &*item_impl.self_ty {
        syn::Type::Path(path) =>
            path.path
                .get_ident()
                .ok_or_else(|| {
                    syn::Error::new_spanned(
                        &item_impl.self_ty,
                        "Expected a simple struct type name"
                    )
                })?,
        _ => {
            return Err(
                syn::Error::new_spanned(&item_impl.self_ty, "Expected a simple struct type name")
            );
        }
    };

    let struct_fake_name = syn::Ident::new(&format!("{}Fake", struct_name), struct_name.span());

    // Process each item in the impl block
    let mut fakes: Vec<proc_macro2::TokenStream> = Vec::new();

    for item in &mut item_impl.items {
        if let syn::ImplItem::Fn(impl_fn) = item {
            // Extract method name
            let fn_name = &impl_fn.sig.ident;
            let fn_fake_name = syn::Ident::new(&format!("{}_fake", fn_name), fn_name.span());

            // Create function pointer type from signature
            let fn_ptr_type = {
                // Skip the receiver (self, &self, &mut self) and get the remaining arguments
                let args: Vec<_> = impl_fn.sig.inputs
                    .iter()
                    .filter(|arg| !matches!(arg, syn::FnArg::Receiver(_)))
                    .collect();
                let output = match &impl_fn.sig.output {
                    syn::ReturnType::Default => quote! { () },
                    syn::ReturnType::Type(_, ty) => quote! { -> #ty },
                };
                quote! { fn(#struct_name, #(#args),*) #output }
            };

            // Create the fake definition
            let fake_block = create_regular_regular_impl_function_fake(
                fn_name,
                &fn_fake_name,
                fn_ptr_type
            );
            fakes.push(fake_block);

            // Insert fake call into the method block
            impl_fn.block = insert_regular_regular_function_fake_call_into_fn_block(
                &impl_fn.block,
                &fn_fake_name,
                &struct_fake_name,
                struct_name,
                &impl_fn.sig.inputs
            );
        }
    }

    // Wrap all fakes with the module
    let fake_module = wrap_regular_impl_function_fakes_with_module(&struct_fake_name, fakes);

    // Generate the expanded output with the updated impl block and fake module
    let expanded = quote! {
        #item_impl

        #fake_module
    };

    Ok(TokenStream::from(expanded))
}
