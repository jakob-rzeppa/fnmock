use crate::strategy::{
    execute, function::fake::FunctionFakeStrategy, impl_block::fake::ImplFakeStrategy,
};

pub fn handle_fakeable(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    match syn::parse2::<syn::Item>(item.clone()) {
        Ok(syn::Item::Fn(item_fn)) => execute::<FunctionFakeStrategy>(item_fn),
        Ok(syn::Item::Impl(item_impl)) => execute::<ImplFakeStrategy>(item_impl),
        Ok(item) => {
            Err(syn::Error::new_spanned(
                item,
                "The #[fakeable] attribute can only be applied to functions and impl blocks.",
            ))
        }
        Err(e) => {
            Err(e)
        }
    }
}
