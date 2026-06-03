use proc_macro::TokenStream;

use crate::fakeable::{ function::fakeable_function, impl_block::fakeable_impl_block };

mod fakeable;

#[proc_macro_attribute]
pub fn fakeable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // First, parse as a generic item to check what type it is
    let res = match syn::parse::<syn::Item>(item.clone()) {
        Ok(syn::Item::Fn(item_fn)) => {
            // If it's a function, process it
            fakeable_function(item_fn)
        }
        Ok(syn::Item::Impl(item_impl)) => {
            // If it's an impl block, process it
            fakeable_impl_block(item_impl)
        }
        Ok(item) =>
            Err(
                syn::Error::new_spanned(
                    item,
                    "The #[fakeable] attribute can only be applied to functions and impl blocks."
                )
            ),
        Err(e) => Err(e),
    };

    match res {
        Ok(expanded) => TokenStream::from(expanded),
        Err(e) => e.to_compile_error().into(),
    }
}
