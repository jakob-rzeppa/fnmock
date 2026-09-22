use crate::{
    entry::dispatch,
    strategy::{function::fake::FunctionFakeStrategy, impl_block::fake::ImplFakeStrategy},
};

pub fn handle_fakeable(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    dispatch::<FunctionFakeStrategy, ImplFakeStrategy>(item)
}
