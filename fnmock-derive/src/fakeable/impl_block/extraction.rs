use quote::quote;
use crate::fakeable::{
    generic_helpers::{
        generate_fake_store_name,
        extract_generic_params,
        extract_generic_idents,
        build_type_id_array,
    },
    impl_block::info::{ GenericFakeableImplFnInfo, FakeableImplFnInfo },
};

pub fn extract_generic_fakeable_impl_fn_info(
    impl_item: &syn::ItemImpl
) -> syn::Result<Vec<FakeableImplFnInfo>> {
    let struct_info = extract_struct_info_from_impl(impl_item)?;

    let mut infos = Vec::new();

    // Iterate over all items in the impl block in order
    for item in &impl_item.items {
        if let syn::ImplItem::Fn(impl_fn) = item {
            let info = extract_info_for_single_method(impl_item, impl_fn, &struct_info)?;
            infos.push(info);
        }
    }

    Ok(infos)
}

/// Extract information for a single method within an impl block
fn extract_info_for_single_method(
    impl_item: &syn::ItemImpl,
    impl_fn: &syn::ImplItemFn,
    struct_info: &StructInfo
) -> syn::Result<FakeableImplFnInfo> {
    let fn_names = extract_fn_names(&impl_fn.sig.ident);

    let fn_info = extract_fn_generic_info(&impl_fn.sig);

    let fn_param_idents = extract_fn_params(&impl_fn.sig);

    let fn_ptr_type = build_fn_ptr_type(impl_item, &impl_fn.sig)?;

    // Only include generic info if there are generics on the struct or method
    let generic_info = if
        !struct_info.generics_idents.is_empty() ||
        !fn_info.generic_idents.is_empty()
    {
        Some(GenericFakeableImplFnInfo {
            struct_generic_idents: struct_info.generics_idents.clone(),
            fn_generic_idents: fn_info.generic_idents.clone(),
            struct_generic_params: struct_info.generics_params.clone(),
            fn_generic_params: fn_info.generic_params.clone(),
            struct_generic_type_ids: struct_info.type_ids.clone(),
            fn_generic_type_ids: fn_info.type_ids.clone(),
        })
    } else {
        None
    };

    Ok(FakeableImplFnInfo {
        fn_name: fn_names.name,
        fake_access_fn_name: fn_names.fake_name,
        fake_store_name: fn_names.store_name,
        fake_api_name: fn_names.fake_api_name,
        fake_module: struct_info.fake_module.clone(),
        fn_param_idents,
        fn_ptr_type,
        generic_info,
    })
}

/// Helper struct for function names
struct FnNames {
    name: syn::Ident,
    fake_name: syn::Ident,
    fake_api_name: syn::Ident,
    store_name: syn::Ident,
}

/// Extract the function name, fake function name, and fake store name
fn extract_fn_names(fn_ident: &syn::Ident) -> FnNames {
    let combined_fake_name = format!("{}_fake", fn_ident.to_string());
    let pascal_fn_name = format_pascal_case(&fn_ident.to_string());

    FnNames {
        name: fn_ident.clone(),
        fake_name: syn::Ident::new(&combined_fake_name, fn_ident.span()),
        fake_api_name: syn::Ident::new(&format!("{}Fake", pascal_fn_name), fn_ident.span()),
        store_name: generate_fake_store_name(fn_ident),
    }
}

/// Convert snake_case to PascalCase
fn format_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Helper struct for struct info extracted from impl
#[derive(Clone)]
struct StructInfo {
    fake_module: syn::Ident,
    generics_idents: Vec<syn::Ident>,
    generics_params: Vec<syn::GenericParam>,
    type_ids: Vec<proc_macro2::TokenStream>,
}

/// Extract struct information including name, generics, and module names
fn extract_struct_info_from_impl(impl_item: &syn::ItemImpl) -> syn::Result<StructInfo> {
    // Extract struct name from self_ty
    let struct_name = match &*impl_item.self_ty {
        syn::Type::Path(path) => {
            // Get the last segment of the path (handles simple and complex paths)
            path.path.segments
                .last()
                .ok_or_else(|| {
                    syn::Error::new_spanned(
                        &impl_item.self_ty,
                        "Expected a simple struct type name"
                    )
                })?
                .ident.clone()
        }
        _ => {
            return Err(
                syn::Error::new_spanned(&impl_item.self_ty, "Expected a simple struct type name")
            );
        }
    };

    // Extract generics from struct
    let struct_generics_params = extract_generic_params(&impl_item.generics);
    let struct_generics_idents = extract_generic_idents(&struct_generics_params);
    let struct_type_ids = build_type_id_array(&struct_generics_idents);

    let fake_module = syn::Ident::new(
        &format!("{}_fake", struct_name.to_string().to_lowercase()),
        struct_name.span()
    );

    Ok(StructInfo {
        fake_module,
        generics_idents: struct_generics_idents,
        generics_params: struct_generics_params
            .into_iter()
            .map(|tp| syn::GenericParam::Type(tp))
            .collect(),
        type_ids: struct_type_ids,
    })
}

/// Helper struct for function generic information
struct FnGenericInfo {
    generic_idents: Vec<syn::Ident>,
    generic_params: Vec<syn::GenericParam>,
    type_ids: Vec<proc_macro2::TokenStream>,
}

/// Extract generic parameters from function signature
fn extract_fn_generic_info(sig: &syn::Signature) -> FnGenericInfo {
    let fn_generics_params = extract_generic_params(&sig.generics);
    let fn_generics_idents = extract_generic_idents(&fn_generics_params);
    let fn_type_ids = build_type_id_array(&fn_generics_idents);

    FnGenericInfo {
        generic_idents: fn_generics_idents,
        generic_params: fn_generics_params
            .into_iter()
            .map(|tp| syn::GenericParam::Type(tp))
            .collect(),
        type_ids: fn_type_ids,
    }
}

/// Extract parameter identifiers from function signature (including receiver)
fn extract_fn_params(sig: &syn::Signature) -> Vec<syn::Ident> {
    sig.inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                syn::FnArg::Receiver(_) => {
                    Some(syn::Ident::new("self", proc_macro2::Span::call_site()))
                }
                syn::FnArg::Typed(pat_type) => {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        Some(pat_ident.ident.clone())
                    } else {
                        None
                    }
                }
            }
        })
        .collect()
}

/// Build the function pointer type for the fake implementation
fn build_fn_ptr_type(impl_item: &syn::ItemImpl, sig: &syn::Signature) -> syn::Result<syn::Type> {
    // Use the self_ty directly - it already has generics if needed
    let self_ty = &impl_item.self_ty;

    // Extract receiver type (handles &self, &mut self, self, etc.)
    let receiver_type = if let Some(syn::FnArg::Receiver(receiver)) = sig.inputs.first() {
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
    let param_types: Vec<_> = sig.inputs
        .iter()
        .skip(1) // Skip receiver
        .collect();

    let output = match &sig.output {
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
            sig,
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
