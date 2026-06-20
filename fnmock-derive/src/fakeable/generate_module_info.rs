use crate::{
    extract::{ function::{ FunctionInfo }, impl_block::{ ItemImplMethodInfo } },
    fakeable::info::{ FakeableGenericInfo, FakeableInfo },
    names::{
        NameType,
        build_impl_interface_struct_name,
        build_impl_module_name,
        build_impl_store_name,
        build_interface_struct_name,
        build_module_name,
        build_store_name,
    },
};

pub fn generate_fakeable_info_from_function(
    function_info: &FunctionInfo
) -> syn::Result<FakeableInfo> {
    let module_name = build_module_name(&function_info.name, NameType::Fake);
    let store_name = build_store_name(&function_info.name, NameType::Fake);
    let display_name = format!("{}", function_info.name);
    let interface_struct_name = build_interface_struct_name(&function_info.name, NameType::Fake);

    Ok(FakeableInfo {
        module_name,
        store_name,
        display_name,
        interface_struct_name,
        fn_ptr_type: function_info.fn_ptr_type.clone(),
        generic_info: function_info.generic_info.as_ref().map(|info| FakeableGenericInfo {
            generic_count: info.count,
            generic_idents: info.idents.clone(),
            generic_params: info.type_params.clone(),
            generic_type_ids: info.type_ids.clone(),
        }),
    })
}

pub fn generate_fakeable_info_from_impl_block(
    item_impl_info: &[ItemImplMethodInfo]
) -> syn::Result<Vec<FakeableInfo>> {
    item_impl_info
        .into_iter()
        .map(|method_info| {
            let module_name = build_impl_module_name(
                &method_info.struct_name,
                &method_info.method_name,
                NameType::Fake
            );
            let store_name = build_impl_store_name(
                &method_info.struct_name,
                &method_info.method_name,
                NameType::Fake
            );
            let display_name = format!("{} {}", method_info.struct_name, method_info.method_name);
            let interface_struct_name = build_impl_interface_struct_name(
                &method_info.struct_name,
                &method_info.method_name,
                NameType::Fake
            );

            Ok(FakeableInfo {
                module_name,
                store_name,
                display_name,
                interface_struct_name,
                fn_ptr_type: method_info.fn_ptr_type.clone(),
                generic_info: method_info.generic_info.as_ref().map(|info| FakeableGenericInfo {
                    generic_count: info.count,
                    generic_params: info.type_params.clone(),
                    generic_idents: info.idents.clone(),
                    generic_type_ids: info.type_ids.clone(),
                }),
            })
        })
        .collect()
}
