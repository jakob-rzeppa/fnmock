use quote::quote;

use crate::spyable::access_function::info::SpyAccessFunctionInfo;

pub fn generate_spy_access_function(info: &SpyAccessFunctionInfo) -> syn::Result<syn::ItemFn> {
    let access_function_name = &info.access_function_name;
    let module_name = &info.module_name;
    let interface_struct_name = &info.interface_struct_name;

    syn::parse2(quote! {
        /// Access the fake for this function, keyed by the given generic arguments.
        ///
        /// Each combination of generic arguments is faked independently, so always specify
        /// the generics explicitly both here and at the call site — if they don't match, the
        /// call silently falls through to the real implementation instead of erroring.
        ///
        /// Only available under `#[cfg(test)]`, and only reachable from tests within this
        /// crate: not from integration tests, doctests, or other crates.
        #[cfg(test)]
        pub(crate) fn #access_function_name() -> self::#module_name::#interface_struct_name {
            self::#module_name::internal_get_interface()
        }
    })
}
