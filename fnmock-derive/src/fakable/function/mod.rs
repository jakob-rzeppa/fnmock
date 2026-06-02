use proc_macro::TokenStream;

pub fn fakable_function(item_fn: syn::ItemFn) -> syn::Result<TokenStream> {
    Ok(TokenStream::new())
}
