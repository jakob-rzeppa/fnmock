use quote::quote;
use crate::param_utils::get_param_names;

/// Generates a fake function that delegates to the fake module's get_implementation method.
///
/// Creates a function with the same signature as the original function,
/// but with `_fake` suffix, that calls the fake implementation.
///
/// # Arguments
///
/// * `fake_fn_name` - The name of the fake function (original name with `_fake` suffix)
/// * `fn_inputs` - The function parameters
/// * `fn_output` - The return type
pub(crate) fn create_fake_function(
    fake_fn_name: syn::Ident,
    fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    fn_output: syn::ReturnType,
) -> proc_macro2::TokenStream {
    let param_names = get_param_names(&fn_inputs);
    
    quote! {
        pub(crate) fn #fake_fn_name(#fn_inputs) #fn_output {
            #fake_fn_name::get_implementation()(#(#param_names),*)
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
pub(crate) fn create_fake_module(fake_fn_name: syn::Ident, params_type: syn::Type, return_type: syn::Type) -> proc_macro2::TokenStream {
    quote! {
        pub(crate) mod #fake_fn_name {
            use fnmock::function_fake::FunctionFake;

            type Function = fn(#params_type) -> #return_type;

            thread_local! {
                static FAKE: std::cell::RefCell<FunctionFake<Function>> =
                    std::cell::RefCell::new(FunctionFake::new(stringify!(#fake_fn_name)));
            }

            pub(crate) fn fake_implementation(new_f: Function) {
                FAKE.with(|fake| { fake.borrow_mut().fake_implementation(new_f) })
            }

            pub(crate) fn clear_fake() {
                FAKE.with(|fake| { fake.borrow_mut().clear_fake() })
            }

            pub(crate) fn get_implementation() -> Function {
                FAKE.with(|fake| { fake.borrow().get_implementation() })
            }
        }
    }
}
