use quote::quote;

use crate::fakeable::{
    function::info::{ FakeableFnGenericInfo, FakeableFnInfo },
    generic_helpers::{
        build_type_id_array,
        extract_generic_idents_from_params,
        extract_generic_type_params,
    },
    helpers::snake_to_pascal_case,
};

pub fn extract_fakeable_fn_info(item_fn: &syn::ItemFn) -> syn::Result<FakeableFnInfo> {
    let (fn_name, fake_access_fn_name, fake_store_name, fake_api_struct_name, fake_module_name) =
        extract_names(&item_fn);

    let (fn_param_idents, fn_param_types) = extract_param_info(&item_fn);

    let fn_ptr_type = build_fn_ptr_type(&fn_param_types, &item_fn.sig.output)?;

    let generic_info = extract_generic_info(&item_fn);

    Ok(FakeableFnInfo {
        fn_name,
        fake_access_fn_name,
        fake_store_name,
        fake_api_struct_name,
        fake_module_name,
        fn_param_idents,
        fn_ptr_type,
        generic_info,
    })
}

fn extract_names(
    item_fn: &syn::ItemFn
) -> (syn::Ident, syn::Ident, syn::Ident, syn::Ident, syn::Ident) {
    let fn_name = item_fn.sig.ident.clone();
    let fake_access_fn_name = format!("{}_fake", fn_name);
    let fake_store_name = format!("{}_FAKE_STORE", fn_name.to_string().to_uppercase());
    let fake_api_struct_name = format!(
        "{}FakeInterface",
        snake_to_pascal_case(&fn_name.to_string())
    );
    let fake_module_name = format!("{}_fake_internal", fn_name);

    (
        fn_name,
        syn::Ident::new(&fake_access_fn_name, item_fn.sig.ident.span()),
        syn::Ident::new(&fake_store_name, item_fn.sig.ident.span()),
        syn::Ident::new(&fake_api_struct_name, item_fn.sig.ident.span()),
        syn::Ident::new(&fake_module_name, item_fn.sig.ident.span()),
    )
}

fn extract_param_info(item_fn: &syn::ItemFn) -> (Vec<syn::Ident>, Vec<syn::Type>) {
    let fn_param_idents = item_fn.sig.inputs
        .iter()
        .filter_map(|input| {
            match input {
                syn::FnArg::Typed(pat_type) =>
                    Some(match pat_type.pat.as_ref() {
                        syn::Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                        _ => {
                            return None;
                        }
                    }),
                _ => None,
            }
        })
        .collect();

    let fn_param_types = item_fn.sig.inputs
        .iter()
        .filter_map(|input| {
            match input {
                syn::FnArg::Typed(pat_type) => Some((*pat_type.ty).clone()),
                _ => None,
            }
        })
        .collect();

    (fn_param_idents, fn_param_types)
}

fn build_fn_ptr_type(
    fn_param_types: &[syn::Type],
    fn_output: &syn::ReturnType
) -> syn::Result<syn::Type> {
    let fn_ptr_tokens = quote! { fn(#(#fn_param_types),*) #fn_output };

    // Parse the token stream into a Type
    syn::parse(fn_ptr_tokens.into()).map_err(|err|
        syn::Error::new_spanned(
            fn_output,
            "Failed to parse function pointer type: ".to_string() + &err.to_string()
        )
    )
}

fn extract_generic_info(item_fn: &syn::ItemFn) -> Option<FakeableFnGenericInfo> {
    if item_fn.sig.generics.params.is_empty() {
        return None;
    }

    let fn_generic_params = extract_generic_type_params(&item_fn.sig.generics);
    let fn_generic_idents = extract_generic_idents_from_params(fn_generic_params.as_slice());

    let fn_generic_type_ids = build_type_id_array(&fn_generic_idents);

    Some(FakeableFnGenericInfo {
        fn_generic_params,
        fn_generic_idents,
        fn_generic_type_ids,
    })
}
