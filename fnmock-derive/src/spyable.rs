use crate::{
    entry::dispatch,
    strategy::{function::spy::FunctionSpyStrategy, impl_block::spy::ImplSpyStrategy},
};

pub fn handle_spyable(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    dispatch::<FunctionSpyStrategy, ImplSpyStrategy>(item)
}
