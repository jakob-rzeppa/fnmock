use quote::quote;

use crate::fakeable::function::info::FakeableFnInfo;

pub fn build_access_function(info: &FakeableFnInfo) -> syn::ItemFn {
    let fake_access_fn_name = &info.fake_access_fn_name;
    let fake_module = &info.fake_module_name;
    let fake_api_name = &info.fake_api_struct_name;

    let (fn_generic_params, fn_generic_idents) = info.generic_info
        .as_ref()
        .map(|generic_info| (
            generic_info.fn_generic_params.clone(),
            generic_info.fn_generic_idents.clone(),
        ))
        .unwrap_or((Vec::new(), Vec::new()));

    syn::parse2(
        quote! {
        #[cfg(test)]
        pub(crate) fn #fake_access_fn_name<#(#fn_generic_params),*>() -> #fake_module::#fake_api_name<#(#fn_generic_idents),*> {
            #fake_module::#fake_api_name::new()
        }
    }
    ).expect("Failed to parse generated access function")
}
