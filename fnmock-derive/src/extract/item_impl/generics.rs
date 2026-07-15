use crate::extract::{
    generic::{
        build_generic_key_array,
        extract_generic_type_and_const_params,
        extract_generic_types_from_generic_params,
    },
    item_impl::info::ImplItemFnGenericInfo,
};

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a impl block method.
///
/// The generics of the struct and method are combined, in the order of struct generics followed by method generics.
pub fn extract_generic_impl_info(
    item_impl: &syn::ItemImpl,
    method: &syn::ImplItemFn
) -> syn::Result<Option<ImplItemFnGenericInfo>> {
    let struct_generic_params = extract_generic_type_and_const_params(&item_impl.generics)?;
    let method_generic_params = extract_generic_type_and_const_params(&method.sig.generics)?;
    let type_params = struct_generic_params.combine(&method_generic_params);

    if type_params.is_empty() {
        return Ok(None);
    }

    let struct_types = extract_generic_types_from_generic_params(&struct_generic_params);
    let method_types = extract_generic_types_from_generic_params(&method_generic_params);
    let types = extract_generic_types_from_generic_params(&type_params);

    let struct_generic_keys = build_generic_key_array(&struct_generic_params);
    let method_generic_keys = build_generic_key_array(&method_generic_params);
    let generic_keys = build_generic_key_array(&type_params);

    Ok(
        Some(ImplItemFnGenericInfo {
            count: type_params.len(),

            generic_params: type_params.to_generic_params(),
            _struct_generic_params: struct_generic_params.to_generic_params(),
            method_generic_params: method_generic_params.to_generic_params(),

            types,
            _struct_types: struct_types,
            _method_types: method_types,

            generic_keys,
            _struct_generic_keys: struct_generic_keys,
            _method_generic_keys: method_generic_keys,
        })
    )
}
