use quote::quote;

/// Builds a function closure trait (e.g. `Fn(i32, &str) -> bool`) from a list of parameter types and a return type.
///
/// Make sure to replace any `Self` types in the parameter types and return type with the actual type of `Self` before calling this function, as it does not handle `Self` replacement itself.
///
/// # Params
///
/// - `lifetime_params`: The lifetime parameters of the function / struct + method signature
/// - `params`: The parameter types of the function
/// - `output`: The return type of the function
pub fn build_fn_closure_trait(
    lifetimes: &[syn::Lifetime],
    params: &[syn::Type],
    output: &syn::ReturnType
) -> syn::Result<syn::TraitBound> {
    let fn_ptr_tokens: proc_macro2::TokenStream = if lifetimes.is_empty() {
        quote! { Fn(#(#params),*) #output }
    } else {
        quote! { for<#(#lifetimes),*> Fn(#(#params),*) #output }
    };
    syn::parse2(fn_ptr_tokens)
}
