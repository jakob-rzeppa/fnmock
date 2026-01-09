use quote::quote;
use crate::function_stub::proxy_docs::StubProxyDocs;

/// Generates a stub function that delegates to the stub module's get_return_value method.
///
/// Creates a function with the same signature as the original function,
/// but with `_stub` suffix, that returns the stubbed value.
///
/// # Arguments
///
/// * `stub_fn_name` - The name of the stub function (original name with `_stub` suffix)
/// * `fn_inputs` - The function parameters
/// * `fn_output` - The return type
pub(crate) fn create_stub_function(
    stub_fn_name: syn::Ident,
    fn_asyncness: Option<syn::token::Async>,
    fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    fn_output: syn::ReturnType,
) -> proc_macro2::TokenStream {
    quote! {
        pub(crate) #fn_asyncness fn #stub_fn_name(#fn_inputs) #fn_output {
            #stub_fn_name::get_return_value()
        }
    }
}

/// Generates a stub module containing the stub infrastructure.
///
/// Creates a module with the same name as the stub function that contains:
/// - Thread-local storage for the FunctionStub instance
/// - Proxy functions for stub operations
///
/// # Arguments
///
/// * `stub_fn_name` - The name of the stub module (same as stub function name)
/// * `return_type` - The return type of the function
pub(crate) fn create_stub_module(stub_fn_name: syn::Ident, return_type: syn::Type) -> proc_macro2::TokenStream {
    // Generate documentation using the proxy_docs module
    let docs = StubProxyDocs::new(&stub_fn_name, &return_type);
    let setup_docs = docs.setup_docs();
    let clear_docs = docs.clear_docs();
    let get_return_value_docs = docs.get_return_value_docs();
    
    quote! {
        pub(crate) mod #stub_fn_name {
            use super::*;

            thread_local! {
                static STUB: std::cell::RefCell<fnmock::function_stub::FunctionStub<#return_type>> =
                    std::cell::RefCell::new(fnmock::function_stub::FunctionStub::new(stringify!(#stub_fn_name)));
            }

            #setup_docs
            pub(crate) fn setup(return_value: #return_type) {
                STUB.with(|stub| { stub.borrow_mut().setup(return_value) })
            }

            #clear_docs
            pub(crate) fn clear() {
                STUB.with(|stub| { stub.borrow_mut().clear() })
            }

            #get_return_value_docs
            pub(crate) fn get_return_value() -> #return_type {
                STUB.with(|stub| { stub.borrow().get_return_value() })
            }
        }
    }
}
