mod access_function;
mod inline_call;
mod spy_module;

pub fn handle_spyable(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    todo!()
}
