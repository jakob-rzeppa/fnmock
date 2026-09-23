use crate::{
    entry::dispatch,
    strategy::{function::mock::FunctionMockStrategy, impl_block::mock::ImplMockStrategy},
};

pub fn handle_mockable(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    dispatch::<FunctionMockStrategy, ImplMockStrategy>(item)
}
