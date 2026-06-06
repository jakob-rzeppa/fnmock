use quote::quote;

use crate::fakeable::impl_block::{ info::FakeableImplFnInfo };

pub fn build_access_fn(fn_info: &FakeableImplFnInfo) -> syn::ImplItemFn {
    let fake_access_fn_name = &fn_info.fake_access_fn_name;
    let fake_module = &fn_info.fake_module;
    let fake_api_name = &fn_info.fake_api_name;

    let (fn_generic_params, fn_generic_idents, struct_generic_idents) = fn_info.generic_info
        .as_ref()
        .map(|generic_info| (
            generic_info.fn_generic_params.clone(),
            generic_info.fn_generic_idents.clone(),
            generic_info.struct_generic_idents.clone(),
        ))
        .unwrap_or((Vec::new(), Vec::new(), Vec::new()));

    // Combine all generic identifiers in the correct order
    let mut all_generic_idents = struct_generic_idents.clone();
    all_generic_idents.extend(fn_generic_idents.clone());

    syn::parse2(
        quote! {
        #[cfg(test)]
        pub(crate) fn #fake_access_fn_name<#(#fn_generic_params),*>() -> #fake_module::#fake_api_name<#(#all_generic_idents),*> {
            #fake_module::#fake_api_name::new()
        }
    }
    ).expect("Failed to parse generated access function")
}
