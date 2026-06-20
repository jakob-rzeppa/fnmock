use quote::quote;

/// Builds a function pointer type (e.g. `fn(i32, &str) -> bool`) from a list of parameter types and a return type.
///
/// Make sure to replace any `Self` types in the parameter types and return type with the actual type of `Self` before calling this function, as it does not handle `Self` replacement itself.
pub fn build_fn_ptr_type(params: &[syn::Type], output: &syn::ReturnType) -> syn::Result<syn::Type> {
    let fn_ptr_tokens = quote! { fn(#(#params),*) #output };
    syn::parse(fn_ptr_tokens.into())
}
