use quote::{ ToTokens, quote };

use crate::fakeable::{
    extract::info::{ FakeableGenericInfo, FakeableInfo },
    generic_helpers::{
        build_type_id_array,
        extract_generic_idents_from_params,
        extract_generic_type_params,
    },
    helpers::{ pascal_to_snake_case, snake_to_pascal_case },
};

pub fn extract_fakeable_info_from_impl_block(
    item_impl: &syn::ItemImpl
) -> syn::Result<Vec<FakeableInfo>> {
    let fakebale_module_info = item_impl.items
        .iter()
        .filter_map(|item| {
            if let syn::ImplItem::Fn(impl_fn) = item {
                Some(extract_fakeable_info_from_single_impl_method(item_impl, impl_fn))
            } else {
                None
            }
        })
        .collect::<syn::Result<Vec<FakeableInfo>>>()?;

    Ok(fakebale_module_info)
}

fn extract_fakeable_info_from_single_impl_method(
    item_impl: &syn::ItemImpl,
    impl_fn: &syn::ImplItemFn
) -> syn::Result<FakeableInfo> {
    let (module_name, store_name, display_name, interface_struct_name) = build_names(
        &item_impl.self_ty, // Convert the self type to a string and remove spaces
        &impl_fn.sig.ident
    );
    let fn_ptr_type = extract_and_build_fn_ptr_type(item_impl, &impl_fn.sig)?;

    let generic_info = extract_generic_info(item_impl, &impl_fn.sig);

    Ok(FakeableInfo {
        module_name,
        store_name,
        display_name,
        interface_struct_name,
        fn_ptr_type,
        generic_info,
    })
}

fn build_names(
    struct_type: &syn::Type,
    method_name: &syn::Ident
) -> (syn::Ident, syn::Ident, String, syn::Ident) {
    let struct_name = struct_type
        .to_token_stream()
        .to_string()
        .replace(' ', "")
        .split('<')
        .next()
        .unwrap_or("")
        .to_string();

    let module_name = syn::Ident::new(
        &format!("{}_{}_fake", pascal_to_snake_case(&struct_name), &method_name.to_string()),
        method_name.span()
    );
    let store_name = syn::Ident::new(
        &format!(
            "{}_{}_FAKE_STORE",
            pascal_to_snake_case(&struct_name).to_uppercase(),
            method_name.to_string().to_uppercase()
        ),
        method_name.span()
    );
    let display_name = format!("{} {} fake", struct_name, method_name);
    let interface_struct_name = syn::Ident::new(
        &format!(
            "{}{}Fake",
            struct_name.to_string(),
            snake_to_pascal_case(&method_name.to_string())
        ),
        method_name.span()
    );

    (module_name, store_name, display_name, interface_struct_name)
}

fn extract_and_build_fn_ptr_type(
    impl_item: &syn::ItemImpl,
    fn_sig: &syn::Signature
) -> syn::Result<syn::Type> {
    // Use the self_ty directly - it already has generics if needed
    let self_ty = &impl_item.self_ty;

    // Extract receiver type (handles &self, &mut self, self, etc.)
    let receiver_type = if let Some(syn::FnArg::Receiver(receiver)) = fn_sig.inputs.first() {
        if receiver.reference.is_some() {
            if receiver.mutability.is_some() {
                Some(quote! { &mut #self_ty })
            } else {
                Some(quote! { &#self_ty })
            }
        } else {
            Some(quote! { #self_ty })
        }
    } else {
        None
    };

    // Get remaining arguments (skip receiver)
    let param_types: Vec<_> = fn_sig.inputs
        .iter()
        .skip(1) // Skip receiver
        .collect();

    let output = match &fn_sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => {
            // Replace Self type with the actual self_ty
            let ty_replaced = replace_self_type(ty, self_ty);
            quote! { -> #ty_replaced }
        }
    };

    // Build the full function pointer type - only include receiver if present
    let fn_ptr_tokens = if let Some(receiver) = receiver_type {
        quote! { fn(#receiver, #(#param_types),*) #output }
    } else {
        quote! { fn(#(#param_types),*) #output }
    };

    // Parse the token stream into a Type
    syn::parse(fn_ptr_tokens.into()).map_err(|err|
        syn::Error::new_spanned(
            fn_sig,
            "Failed to parse function pointer type: ".to_string() + &err.to_string()
        )
    )
}

/// Replace Self type with the actual self_ty
fn replace_self_type(ty: &syn::Type, self_ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Path(type_path) => {
            // Check if this is the Self type
            if type_path.path.segments.len() == 1 && type_path.path.segments[0].ident == "Self" {
                self_ty.clone()
            } else {
                // Recursively replace Self in nested types
                syn::Type::Path(syn::TypePath {
                    qself: type_path.qself.clone(),
                    path: type_path.path.clone(),
                })
            }
        }
        _ => ty.clone(),
    }
}

fn extract_generic_info(
    item_impl: &syn::ItemImpl,
    fn_sig: &syn::Signature
) -> Option<FakeableGenericInfo> {
    if fn_sig.generics.params.is_empty() && item_impl.generics.params.is_empty() {
        return None;
    }

    let struct_generic_params = extract_generic_type_params(&item_impl.generics);
    let fn_generic_params = extract_generic_type_params(&fn_sig.generics);
    let generic_params = struct_generic_params
        .into_iter()
        .chain(fn_generic_params.into_iter())
        .collect::<Vec<_>>();

    let generic_idents = extract_generic_idents_from_params(&generic_params);
    let generic_type_ids = build_type_id_array(&generic_idents);

    Some(FakeableGenericInfo {
        generic_count: generic_params.len(),
        generic_params,
        generic_idents,
        generic_type_ids,
    })
}
