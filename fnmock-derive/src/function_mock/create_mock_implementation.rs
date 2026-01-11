use quote::quote;
use crate::function_mock::proxy_docs::MockProxyDocs;

/// Generates the original function with mock checking logic injected.
///
/// Creates a function that first checks (in test mode) if a mock implementation has been
/// configured via the mock module. If a mock is set, it calls the mock implementation.
/// Otherwise, it executes the original function body.
///
/// # Arguments
///
/// * `fn_name` - The name of the original function
/// * `fn_visibility` - The visibility modifier of the function (pub, pub(crate), etc.)
/// * `fn_asyncness` - Optional async keyword if the function is async
/// * `fn_inputs` - The function parameters
/// * `fn_output` - The return type
/// * `fn_block` - The original function body to execute when mock is not set
/// * `mock_mod_name` - The name of the mock module containing the mock infrastructure
/// * `params_to_tuple` - Token stream that converts parameters into a tuple for the mock
///
/// # Returns
///
/// Generated token stream for the function with injected mock checking logic
pub(crate) fn create_mock_function(
    fn_name: syn::Ident,
    fn_visibility: syn::Visibility,
    fn_asyncness: Option<syn::token::Async>,
    fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    fn_output: syn::ReturnType,
    fn_block: Box<syn::Block>,
    mock_mod_name: syn::Ident,
    params_to_tuple: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let original_fn_stmts = &fn_block.stmts;
    
    quote! {
        #[allow(unused_variables)]
        #fn_visibility #fn_asyncness fn #fn_name(#fn_inputs) #fn_output {
            // Call the mock implementation if set (only in test mode)
            #[cfg(test)]
            if #mock_mod_name::is_set() {
                return #mock_mod_name::call(#params_to_tuple);
            }

            #(#original_fn_stmts)*
        }
    }
}

/// Generates a mock module containing the mock infrastructure.
///
/// Creates a module with the same name as the mock function that contains:
/// - Type aliases for parameters and return type
/// - Thread-local storage for the FunctionMock instance
/// - Proxy functions for all mock operations
///
/// # Arguments
///
/// * `mock_fn_name` - The name of the mock module (same as mock function name)
/// * `params_type` - The type representing the function parameters (single type or tuple)
/// * `return_type` - The return type of the function
/// * `fn_inputs` - The original function parameters (for documentation)
/// * `ignore_indices` - Indices of parameters to ignore (for documentation)
/// * `params_to_tuple` - Token stream that converts parameters into a tuple
/// * `filtered_fn_inputs` - Function parameters excluding ignored ones
pub(crate) fn create_mock_module(
    mock_fn_name: syn::Ident,
    params_type: syn::Type,
    return_type: syn::Type,
    fn_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    ignore_indices: &[usize],
    fn_asyncness: Option<syn::token::Async>,
    params_to_tuple: proc_macro2::TokenStream,
    filtered_fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    // Generate documentation using the proxy_docs module
    let docs = MockProxyDocs::new(&mock_fn_name, fn_inputs, ignore_indices, &return_type, fn_asyncness);
    let call_docs = docs.call_docs();
    let setup_docs = docs.setup_docs();
    let clear_docs = docs.clear_docs();
    let is_set_docs = docs.is_set_docs();
    let assert_times_docs = docs.assert_times_docs();
    let assert_with_docs = docs.assert_with_docs();

    quote! {
        pub(crate) mod #mock_fn_name {
            use super::*;

            thread_local! {
                static MOCK: std::cell::RefCell<fnmock::function_mock::FunctionMock<
                    #params_type,
                    #return_type,
                >> = std::cell::RefCell::new(fnmock::function_mock::FunctionMock::new(stringify!(#mock_fn_name)));
            }

            #call_docs
            pub(crate) fn call(params: #params_type) -> #return_type {
                MOCK.with(|mock| {
                    mock.borrow_mut().call(params)
                })
            }

            #setup_docs
            pub(crate) fn setup(new_f: fn(#params_type) -> #return_type) {
                MOCK.with(|mock| {
                    mock.borrow_mut().setup(new_f)
                })
            }

            #clear_docs
            pub(crate) fn clear() {
                MOCK.with(|mock|{
                    mock.borrow_mut().clear()
                })
            }

            #is_set_docs
            pub(crate) fn is_set() -> bool {
                MOCK.with(|mock| {
                    mock.borrow().is_set()
                })
            }

            #assert_times_docs
            pub(crate) fn assert_times(expected_num_of_calls: u32) {
                MOCK.with(|mock| {
                    mock.borrow().assert_times(expected_num_of_calls)
                })
            }

            #assert_with_docs
            pub(crate) fn assert_with(#filtered_fn_inputs) {
                MOCK.with(|mock| {
                    mock.borrow().assert_with(#params_to_tuple)
                })
            }
        }
    }
}