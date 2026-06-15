use quote::quote;

use crate::fakeable::{ inline_call::insert_inline_call_into_fn_block };

mod extract;
mod generic_helpers;
mod helpers;
mod fake_module;
mod inline_call;

pub fn handle_fakeable(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> syn::Result<proc_macro2::TokenStream> {
    // First, parse the input to get the necessary information for creating the fake modules
    // For free functions, we only create one module, but for impl blocks, we may need to create multiple modules (one per method)
    let expanded = match syn::parse::<syn::Item>(item.clone()) {
        Ok(syn::Item::Fn(mut item_fn)) => {
            // If it's a function, extract the fake module info for that function
            let info = extract::function::extract_fakeable_info_from_fn(&item_fn)?;

            // Create the fake module code based on the extracted information
            let module = fake_module::generate_fake_module_code(&info)?;

            let param_idents: Vec<syn::Ident> = item_fn.sig.inputs
                .iter()
                .filter_map(|input| {
                    match input {
                        syn::FnArg::Typed(pat_type) => {
                            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                Some(pat_ident.ident.clone())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
                .collect();

            // Insert the inline call into the original function's block
            let modified_block = insert_inline_call_into_fn_block(
                &item_fn.block,
                &param_idents,
                &info.module_name,
                &info.interface_struct_name,
                info.generic_info.as_ref().map(|gi| &gi.generic_idents[..])
            );
            item_fn.block = Box::new(modified_block);

            quote! {
                #item_fn

                #module
            }
        }
        Ok(syn::Item::Impl(item_impl)) => {
            // If it's an impl block, extract fake module info for each method
            let infos = extract::impl_block::extract_fakeable_info_from_impl_block(&item_impl)?;

            // Create the fake module code based on the extracted information for each method
            let modules = infos
                .iter()
                .map(|info| fake_module::generate_fake_module_code(&info))
                .collect::<syn::Result<Vec<_>>>()?;

            // Insert the inline call into each method's block
            let mut modified_impl = item_impl.clone();

            // We need to iterate over the methods in the impl block and the corresponding info in parallel. We can assume that the order of the info matches the order of the methods in the impl block, since we extract them in that order.
            let mut info_iter = infos.iter();

            for method in &mut modified_impl.items {
                if let syn::ImplItem::Fn(ref mut method_fn) = *method {
                    let info = info_iter
                        .next()
                        .expect("Mismatch between number of methods and extracted info");

                    let param_idents: Vec<syn::Ident> = method_fn.sig.inputs
                        .iter()
                        .filter_map(|input| {
                            match input {
                                syn::FnArg::Typed(pat_type) => {
                                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                        Some(pat_ident.ident.clone())
                                    } else {
                                        None
                                    }
                                }
                                syn::FnArg::Receiver(_) =>
                                    Some(syn::Ident::new("self", method_fn.sig.ident.span())),
                            }
                        })
                        .collect();

                    // Insert the inline call into the method's block
                    method_fn.block = insert_inline_call_into_fn_block(
                        &method_fn.block,
                        &param_idents,
                        &info.module_name,
                        &info.interface_struct_name,
                        info.generic_info.as_ref().map(|gi| &gi.generic_idents[..])
                    );
                }
            }

            quote! {
                #modified_impl

                #(#modules)*
            }
        }
        Ok(item) => {
            return Err(
                syn::Error::new_spanned(
                    item,
                    "The #[fakeable] attribute can only be applied to functions and impl blocks."
                )
            );
        }
        Err(e) => {
            return Err(e);
        }
    };

    Ok(expanded)
}
