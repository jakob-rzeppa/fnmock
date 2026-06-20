use crate::{
    extract::impl_block::extract_item_impl_info,
    fakeable::extract::info::{ FakeableGenericInfo, FakeableInfo },
    names::{
        NameType,
        build_impl_interface_struct_name,
        build_impl_module_name,
        build_impl_store_name,
    },
};

pub fn extract_fakeable_info_from_impl_block(
    item_impl: &syn::ItemImpl
) -> syn::Result<Vec<FakeableInfo>> {
    let item_impl_info = extract_item_impl_info(item_impl)?;

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
                fn_ptr_type: method_info.fn_ptr_type,
                generic_info: method_info.generic_info.map(|info| FakeableGenericInfo {
                    generic_count: info.count,
                    generic_params: info.type_params,
                    generic_idents: info.idents,
                    generic_type_ids: info.type_ids,
                }),
            })
        })
        .collect()
}
