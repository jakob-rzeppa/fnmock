use crate::{ fakeable::handle_fakeable };

mod fakeable;
mod module_builder;
mod names;
mod extract;

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
