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
    // This is the only place proc_macro::TokenStream should appear: the actual proc-macro ABI
    // boundary requires it, but proc_macro::TokenStream cannot be constructed or parsed outside
    // a live macro expansion (it panics), which makes anything using it untestable. Converting to
    // proc_macro2::TokenStream here lets the rest of the crate be tested with ordinary unit tests.
    let res = handle_fakeable(attr.into(), item.into());

    match res {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
