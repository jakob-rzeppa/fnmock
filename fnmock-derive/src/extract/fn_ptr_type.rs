use quote::quote;

/// Builds a function pointer type (e.g. `fn(i32, &str) -> bool`) from a list of parameter types and a return type.
///
/// Make sure to replace any `Self` types in the parameter types and return type with the actual type of `Self` before calling this function, as it does not handle `Self` replacement itself.
///
/// # Params
///
/// - `lifetime_params`: The lifetime parameters of the function / struct + method signature
/// - `params`: The parameter types of the function
/// - `output`: The return type of the function
pub fn build_fn_ptr_type(
    lifetimes: &[syn::Lifetime],
    params: &[syn::Type],
    output: &syn::ReturnType
) -> syn::Result<syn::Type> {
    let fn_ptr_tokens = if lifetimes.is_empty() {
        quote! { fn(#(#params),*) #output }
    } else {
        quote! { for<#(#lifetimes),*> fn(#(#params),*) #output }
    };
    syn::parse(fn_ptr_tokens.into())
}
