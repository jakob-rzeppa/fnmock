use quote::quote;
use syn::__private::TokenStream2;
use crate::function_fake::create_fake_implementation::{create_fake_function, create_fake_module};
use crate::param_utils::{create_param_type, get_param_names};
use crate::return_utils::extract_return_type;

mod create_fake_implementation;
mod proxy_docs;

/// Processes a function and generates the complete fake infrastructure.
///
/// This is the main entry point for the fake_function attribute macro. It takes a function
/// definition and generates:
/// 1. The original function with fake checking logic injected (in test mode, checks if a fake
///    is configured and calls it; otherwise executes the original implementation)
/// 2. A fake module with control methods (test-only) containing `setup()`, `clear()`, `is_set()`,
///    and `get_implementation()` functions
///
/// # Arguments
///
/// * `fake_function` - The function item to create fakes for
///
/// # Returns
///
/// - `Ok(TokenStream2)` - The complete generated code including original and fake infrastructure
/// - `Err(syn::Error)` - If validation fails or the function cannot be faked
pub(crate) fn process_fake_function(fake_function: syn::ItemFn) -> syn::Result<TokenStream2> {
    // Extract function details
    let fn_visibility = fake_function.vis.clone();
    let fn_asyncness = fake_function.sig.asyncness;
    let fn_name = fake_function.sig.ident.clone();
    let fn_inputs = fake_function.sig.inputs.clone();
    let fn_output = fake_function.sig.output.clone();
    let fn_block = fake_function.block.clone();

    // Generate fake function name
    let fake_mod_name = syn::Ident::new(&format!("{}_fake", &fn_name), fn_name.span());

    let params_type = create_param_type(&fn_inputs, &[]);
    let return_type = extract_return_type(&fake_function.sig.output);

    let fake_function = create_fake_function(
        fn_name,
        fn_visibility,
        fn_asyncness,
        fn_inputs.clone(),
        fn_output,
        fn_block,
        fake_mod_name.clone(),
    );

    let fake_module = create_fake_module(
        fake_mod_name,
        params_type,
        return_type,
        &fn_inputs,
        fn_asyncness
    );

    Ok(quote! {
        #fake_function

        #[cfg(test)]
        #fake_module
    })
}
