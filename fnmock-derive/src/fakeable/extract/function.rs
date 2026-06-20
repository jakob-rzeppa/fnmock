use quote::quote;

use crate::{
    fakeable::extract::info::{ FakeableGenericInfo, FakeableInfo },
    generic_helpers::{
        build_type_id_array,
        extract_generic_idents_from_params,
        extract_generic_type_params,
    },
    helpers::snake_to_pascal_case,
    names::{ NameType, build_interface_struct_name, build_module_name, build_store_name },
};

pub fn extract_fakeable_info_from_fn(item_fn: &syn::ItemFn) -> syn::Result<FakeableInfo> {
    let (module_name, store_name, display_name, interface_struct_name) = build_names(
        &item_fn.sig.ident
    );

    let fn_ptr_type = extract_and_build_fn_ptr_type(&item_fn.sig);

    let generic_info = extract_generic_info(&item_fn.sig);

    Ok(FakeableInfo {
        module_name,
        store_name,
        display_name,
        interface_struct_name,
        fn_ptr_type,
        generic_info,
    })
}

fn build_names(fn_name: &syn::Ident) -> (syn::Ident, syn::Ident, String, syn::Ident) {
    let module_name = build_module_name(fn_name, NameType::Fake);
    let store_name = build_store_name(fn_name, NameType::Fake);
    let display_name = format!("{}", fn_name);
    let interface_struct_name = build_interface_struct_name(fn_name, NameType::Fake);

    (module_name, store_name, display_name, interface_struct_name)
}

fn extract_and_build_fn_ptr_type(fn_sig: &syn::Signature) -> syn::Type {
    let fn_param_types: Vec<syn::Type> = fn_sig.inputs
        .iter()
        .filter_map(|input| {
            match input {
                syn::FnArg::Typed(pat_type) => Some((*pat_type.ty).clone()),
                _ => None,
            }
        })
        .collect();

    let fn_output = &fn_sig.output;

    let fn_ptr_tokens = quote! { fn(#(#fn_param_types),*) #fn_output };
    syn::parse(fn_ptr_tokens.into()).expect("Failed to parse function pointer type")
}

fn extract_generic_info(fn_sig: &syn::Signature) -> Option<FakeableGenericInfo> {
    if fn_sig.generics.params.is_empty() {
        return None;
    }

    let generic_params = extract_generic_type_params(&fn_sig.generics);
    let generic_idents = extract_generic_idents_from_params(generic_params.as_slice());

    let generic_type_ids = build_type_id_array(&generic_idents);

    Some(FakeableGenericInfo {
        generic_count: generic_params.len(),
        generic_params,
        generic_idents,
        generic_type_ids,
    })
}
