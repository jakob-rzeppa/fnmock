use crate::extract::{
    generic::{
        build_type_id_array,
        extract_generic_type_params,
        extract_generic_types_from_type_params,
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
    let struct_type_params = extract_generic_type_params(&item_impl.generics)?;
    let method_type_params = extract_generic_type_params(&method.sig.generics)?;
    let type_params = struct_type_params
        .clone()
        .into_iter()
        .chain(method_type_params.clone().into_iter())
        .collect::<Vec<_>>();

    if type_params.is_empty() {
        return Ok(None);
    }

    let struct_types = extract_generic_types_from_type_params(&struct_type_params);
    let method_types = extract_generic_types_from_type_params(&method_type_params);
    let types = extract_generic_types_from_type_params(&type_params);

    let struct_type_ids = build_type_id_array(&struct_types);
    let method_type_ids = build_type_id_array(&method_types);
    let type_ids = build_type_id_array(&types);

    Ok(
        Some(ImplItemFnGenericInfo {
            count: type_params.len(),

            type_params,
            _struct_type_params: struct_type_params,
            method_type_params,

            types,
            _struct_types: struct_types,
            _method_types: method_types,

            type_ids,
            _struct_type_ids: struct_type_ids,
            _method_type_ids: method_type_ids,
        })
    )
}
