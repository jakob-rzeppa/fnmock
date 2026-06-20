use crate::{ fake::handle_fake, fakeable::handle_fakeable };

mod fakeable;
mod fake;
mod helpers;
mod generic_helpers;
mod module_builder;

#[proc_macro_attribute]
pub fn fakeable(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let res = handle_fakeable(attr.into(), item.into());

    match res {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn fake(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    handle_fake(input)
}
