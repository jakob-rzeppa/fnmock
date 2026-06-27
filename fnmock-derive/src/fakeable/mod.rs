use quote::quote;

use crate::{
    extract::{ function::extract_function_info, impl_block::extract_item_impl_info },
    fakeable::{
        generate_module_info::{
            generate_fakeable_info_from_function,
            generate_fakeable_info_from_impl_block,
        },
        inline_call::insert_inline_call_into_fn_block,
    },
};

mod fake_module;
mod inline_call;
mod generate_module_info;
mod info;
mod access_function;

pub fn handle_fakeable(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> syn::Result<proc_macro2::TokenStream> {
    // First, parse the input to get the necessary information for creating the fake modules
    // For free functions, we only create one module, but for impl blocks, we may need to create multiple modules (one per method)
    let expanded = match syn::parse::<syn::Item>(item.clone()) {
        Ok(syn::Item::Fn(mut item_fn)) => {
            // If it's a function, extract the fake module info for that function
            let function_info = extract_function_info(&item_fn)?;
            let info = generate_fakeable_info_from_function(&function_info)?;

            // Create the fake module code based on the extracted information
            let module = fake_module::generate_fake_module_code(&info)?;

            // Insert the inline call into the original function's block
            let modified_block = insert_inline_call_into_fn_block(
                &item_fn.block,
                &function_info.param_idents,
                &info.module_name,
                &info.interface_struct_name,
                info.generic_info.as_ref().map(|gi| &gi.generic_idents[..])
            );
            item_fn.block = Box::new(modified_block);

            // Generate the access function
            let access_function = access_function::generate_access_function_for_standalone(&info)?;

            quote! {
                #item_fn

                #access_function

                #module
            }
        }
        Ok(syn::Item::Impl(item_impl)) => {
            // If it's an impl block, extract fake module info for each method
            let item_impl_info = extract_item_impl_info(&item_impl)?;
            let infos = generate_fakeable_info_from_impl_block(&item_impl_info)?;

            // Create the fake module code based on the extracted information for each method
            let modules = infos
                .iter()
                .map(|info| fake_module::generate_fake_module_code(&info))
                .collect::<syn::Result<Vec<_>>>()?;

            // Generate the access methods for the impl block
            let access_methods = access_function::generate_access_methods_for_impl_block(
                &infos,
                &item_impl_info
            )?;

            // Insert the inline call into each method's block
            let mut modified_impl = item_impl.clone();

            // We need to iterate over the methods in the impl block and the corresponding info in parallel. We can assume that the order of the info matches the order of the methods in the impl block, since we extract them in that order.
            let mut info_iter = infos.iter();

            for method in &mut modified_impl.items {
                if let syn::ImplItem::Fn(ref mut method_fn) = *method {
                    let item_impl_method_info = item_impl_info
                        .iter()
                        .find(|info| info.method_name == method_fn.sig.ident)
                        .expect("Could not find matching method info for method in impl block");

                    let info = info_iter
                        .next()
                        .expect("Mismatch between number of methods and extracted info");

                    // Insert the inline call into the method's block
                    method_fn.block = insert_inline_call_into_fn_block(
                        &method_fn.block,
                        &item_impl_method_info.param_idents,
                        &info.module_name,
                        &info.interface_struct_name,
                        info.generic_info.as_ref().map(|gi| &gi.generic_idents[..])
                    );
                }
            }

            quote! {
                #modified_impl

                #access_methods

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
