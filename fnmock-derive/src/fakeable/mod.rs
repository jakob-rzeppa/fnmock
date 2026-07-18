//! Expansion of the `#[fakeable]` attribute.
//!
//! The attribute expands into three pieces per faked function, one per submodule here:
//!
//! - `fake_module` — a `#[cfg(test)]` module holding the thread-local store and the interface
//!   struct with `setup`/`clear`/`is_set`.
//! - `inline_call` — the lookup injected at the top of the original body, which runs the fake
//!   instead of the body when one is installed.
//! - `access_function` — the accessor tests call to reach the interface struct, e.g.
//!   `fetch_user_fake()`.

use quote::quote;

use crate::{
    extract::{function::extract_function_info, item_impl::extract_item_impl_info},
    fakeable::{
        access_function::info::AccessFunctionInfo, fake_module::info::FakeModuleInfo,
        inline_call::info::InlineCallInfo,
    },
};

mod access_function;
mod fake_module;
mod inline_call;

/// Expand the `#[fakeable]` attribute over the item it was applied to.
///
/// Takes `proc_macro2::TokenStream`s rather than `proc_macro::TokenStream`s so that expansion can
/// be exercised by ordinary unit tests; see the note at the ABI boundary in `lib.rs`.
///
/// # Errors
///
/// Returns a spanned error if the item is neither a function nor an inherent impl block, if it
/// fails to parse, or if any signature involved cannot be faked.
pub fn handle_fakeable(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    // First, parse the input to get the necessary information for creating the fake modules
    // For free functions, we only create one module, but for impl blocks, we may need to create multiple modules (one per method)
    let expanded = match syn::parse2::<syn::Item>(item.clone()) {
        Ok(syn::Item::Fn(mut item_fn)) => {
            // If it's a function, extract the fake info for that function
            let function_info = extract_function_info(&item_fn)?;
            let inline_call_info = InlineCallInfo::try_from(&function_info)?;
            let access_function_info = AccessFunctionInfo::try_from(&function_info)?;
            let fake_module_info = FakeModuleInfo::try_from(&function_info)?;

            // Create the fake module code based on the extracted information
            let module = fake_module::generate::generate_fake_module_code(&fake_module_info)?;

            // Insert the inline call into the original function's block
            let modified_block = inline_call::generate::insert_inline_call_into_fn_block(
                &item_fn.block,
                &inline_call_info,
            )?;
            *item_fn.block = modified_block;

            // Generate the access function
            let access_function =
                access_function::generate::standalone::generate_access_function_for_standalone(
                    &access_function_info,
                )?;

            quote! {
                #item_fn

                #access_function

                #module
            }
        }
        Ok(syn::Item::Impl(item_impl)) => {
            // If it's an impl block, extract fake info for each method
            let item_impl_info = extract_item_impl_info(&item_impl)?;

            let inline_call_infos = item_impl_info
                .iter()
                .map(InlineCallInfo::try_from)
                .collect::<syn::Result<Vec<_>>>()?;
            let access_function_infos = item_impl_info
                .iter()
                .map(AccessFunctionInfo::try_from)
                .collect::<syn::Result<Vec<_>>>()?;
            let fake_module_infos = item_impl_info
                .iter()
                .map(FakeModuleInfo::try_from)
                .collect::<syn::Result<Vec<_>>>()?;

            // Create the fake module code based on the extracted information for each method
            let modules = fake_module_infos
                .iter()
                .map(fake_module::generate::generate_fake_module_code)
                .collect::<syn::Result<Vec<_>>>()?;

            // Generate the access methods for the impl block
            let access_methods =
                access_function::generate::impl_block::generate_access_methods_for_impl_block(
                    &item_impl,
                    &access_function_infos,
                )?;

            // Insert the inline call into each method's block
            let mut modified_impl = item_impl.clone();

            // We need to iterate over the methods in the impl block and the corresponding info in parallel. We can assume that the order of the info matches the order of the methods in the impl block, since we extract them in that order.
            let mut inline_call_info_iter = inline_call_infos.iter();

            for method in &mut modified_impl.items {
                if let syn::ImplItem::Fn(ref mut method_fn) = *method {
                    let inline_call_info = inline_call_info_iter
                        .next()
                        .ok_or_else(||
                            syn::Error::new_spanned(
                                &method_fn.sig.ident,
                                "internal error: more impl-block methods than extracted fake infos while inserting fake lookups. This is a bug in fnmock; please report it."
                            )
                        )?;

                    // Insert the inline call into the method's block
                    method_fn.block = inline_call::generate::insert_inline_call_into_fn_block(
                        &method_fn.block,
                        inline_call_info,
                    )?;
                }
            }

            quote! {
                #modified_impl

                #access_methods

                #(#modules)*
            }
        }
        Ok(item) => {
            return Err(syn::Error::new_spanned(
                item,
                "The #[fakeable] attribute can only be applied to functions and impl blocks.",
            ));
        }
        Err(e) => {
            return Err(e);
        }
    };

    Ok(expanded)
}
