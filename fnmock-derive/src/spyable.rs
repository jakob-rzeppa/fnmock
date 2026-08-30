use crate::strategy::{
    execute, function::spy::FunctionSpyStrategy, impl_block::spy::ImplSpyStrategy,
};

pub fn handle_spyable(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    match syn::parse2::<syn::Item>(item.clone()) {
        Ok(syn::Item::Fn(item_fn)) => execute::<FunctionSpyStrategy>(item_fn),
        Ok(syn::Item::Impl(item_impl)) => execute::<ImplSpyStrategy>(item_impl),
        Ok(item) => {
            Err(syn::Error::new_spanned(
                item,
                "The #[spyable] attribute can only be applied to functions and impl blocks.",
            ))
        }
        Err(e) => {
            Err(e)
        }
    }
}
