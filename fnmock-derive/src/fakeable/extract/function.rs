use crate::{
    extract::function::extract_function_info,
    fakeable::extract::info::{ FakeableGenericInfo, FakeableInfo },
    names::{ NameType, build_interface_struct_name, build_module_name, build_store_name },
};

pub fn extract_fakeable_info_from_fn(item_fn: &syn::ItemFn) -> syn::Result<FakeableInfo> {
    let function_info = extract_function_info(item_fn)?;

    let module_name = build_module_name(&function_info.name, NameType::Fake);
    let store_name = build_store_name(&function_info.name, NameType::Fake);
    let display_name = format!("{}", function_info.name);
    let interface_struct_name = build_interface_struct_name(&function_info.name, NameType::Fake);

    Ok(FakeableInfo {
        module_name,
        store_name,
        display_name,
        interface_struct_name,
        fn_ptr_type: function_info.fn_ptr_type,
        generic_info: function_info.generic_info.map(|info| FakeableGenericInfo {
            generic_count: info.generic_count,
            generic_idents: info.generic_idents,
            generic_params: info.generic_type_params,
            generic_type_ids: info.generic_type_ids,
        }),
    })
}
