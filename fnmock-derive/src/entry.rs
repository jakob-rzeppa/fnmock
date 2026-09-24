use crate::strategy::{Strategy, execute};

/// Parses `item` and dispatches it to whichever of `F` / `I` matches its shape: a free function
/// goes to `F`, an inherent impl block to `I`. Shared by every attribute so each one only has to
/// name its two strategies.
pub fn dispatch<F, I>(item: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream>
where
    F: Strategy<Item = syn::ItemFn>,
    I: Strategy<Item = syn::ItemImpl>,
{
    match syn::parse2::<syn::Item>(item) {
        Ok(syn::Item::Fn(item_fn)) => execute::<F>(item_fn),
        Ok(syn::Item::Impl(item_impl)) => execute::<I>(item_impl),
        Ok(item) => Err(syn::Error::new_spanned(
            item,
            "The macro can only be applied to functions and impl blocks.",
        )),
        Err(e) => Err(e),
    }
}
