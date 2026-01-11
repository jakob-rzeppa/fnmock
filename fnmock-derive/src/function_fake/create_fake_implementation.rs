use quote::quote;
use syn::token::Async;
use crate::param_utils::get_param_names;
use crate::function_fake::proxy_docs::FakeProxyDocs;

/// Generates the original function with fake checking logic injected.
///
/// Creates a function that first checks (in test mode) if a fake implementation has been
/// configured via the fake module. If a fake is set, it calls the fake implementation.
/// Otherwise, it executes the original function body.
///
/// # Arguments
///
/// * `fn_name` - The name of the original function
/// * `fn_visibility` - The visibility modifier of the function (pub, pub(crate), etc.)
/// * `fn_asyncness` - Optional async keyword if the function is async
/// * `fn_inputs` - The function parameters
/// * `fn_output` - The return type
/// * `fn_block` - The original function body to execute when fake is not set
/// * `fake_mod_name` - The name of the fake module containing the fake infrastructure
///
/// # Returns
///
/// Generated token stream for the function with injected fake checking logic
pub(crate) fn create_fake_function(
    fn_name: syn::Ident,
    fn_visibility: syn::Visibility,
    fn_asyncness: Option<Async>,
    fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    fn_output: syn::ReturnType,
    fn_block: Box<syn::Block>,
    fake_mod_name: syn::Ident,
) -> proc_macro2::TokenStream {
    let param_names = get_param_names(&fn_inputs);
    let original_fn_stmts = &fn_block.stmts;
    
    quote! {
        #fn_visibility #fn_asyncness fn #fn_name(#fn_inputs) #fn_output {
            // Call the fake implementation if set (only in test mode)
            #[cfg(test)]
            if #fake_mod_name::is_set() {
                return #fake_mod_name::get_implementation()(#(#param_names),*);
            }

            #(#original_fn_stmts)*
        }
    }
}

/// Generates a fake module containing the fake infrastructure.
///
/// Creates a module with the same name as the fake function that contains:
/// - Type alias for the function type
/// - Thread-local storage for the FunctionFake instance
/// - Proxy functions for fake operations
///
/// # Arguments
///
/// * `fake_fn_name` - The name of the fake module (same as fake function name)
/// * `params_type` - The type representing the function parameters (single type or tuple)
/// * `return_type` - The return type of the function
/// * `fn_inputs` - The original function parameters (for documentation)
pub(crate) fn create_fake_module(
    fake_fn_name: syn::Ident,
    params_type: syn::Type,
    return_type: syn::Type,
    fn_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    fn_asyncness: Option<syn::token::Async>,
) -> proc_macro2::TokenStream {
    // Generate documentation using the proxy_docs module
    let docs = FakeProxyDocs::new(&fake_fn_name, fn_inputs, &return_type, fn_asyncness);
    let setup_docs = docs.setup_docs();
    let clear_docs = docs.clear_docs();
    let is_set_docs = docs.is_set_docs();
    let get_implementation_docs = docs.get_implementation_docs();
    
    quote! {
        pub(crate) mod #fake_fn_name {
            use super::*;

            thread_local! {
                static FAKE: std::cell::RefCell<fnmock::function_fake::FunctionFake<fn(#params_type) -> #return_type>> =
                    std::cell::RefCell::new(fnmock::function_fake::FunctionFake::new(stringify!(#fake_fn_name)));
            }

            #setup_docs
            pub(crate) fn setup(new_f: fn(#params_type) -> #return_type) {
                FAKE.with(|fake| { fake.borrow_mut().setup(new_f) })
            }

            #clear_docs
            pub(crate) fn clear() {
                FAKE.with(|fake| { fake.borrow_mut().clear() })
            }

            #is_set_docs
            pub(crate) fn is_set() -> bool {
                FAKE.with(|fake| { fake.borrow().is_set() })
            }

            #get_implementation_docs
            pub(crate) fn get_implementation() -> fn(#params_type) -> #return_type {
                FAKE.with(|fake| { fake.borrow().get_implementation() })
            }
        }
    }
}
