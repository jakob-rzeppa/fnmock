use proc_macro::TokenStream;

pub fn fakable_impl_block(item_impl: syn::ItemImpl) -> syn::Result<TokenStream> {
    Ok(TokenStream::new())
}
