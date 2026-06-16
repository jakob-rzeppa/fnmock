use quote::{ ToTokens, quote };
use syn::visit_mut::VisitMut;

use crate::{
    fakeable::extract::{ info::{ FakeableGenericInfo, FakeableInfo } },
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
    let fn_ptr_type = extract_and_build_fn_ptr_type(item_impl, &impl_fn.sig);

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
            "{}{}FakeInterface",
            struct_name.to_string(),
            snake_to_pascal_case(&method_name.to_string())
        ),
        method_name.span()
    );

    (module_name, store_name, display_name, interface_struct_name)
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

fn extract_and_build_fn_ptr_type(impl_item: &syn::ItemImpl, fn_sig: &syn::Signature) -> syn::Type {
    let fn_param_types: Vec<syn::Type> = fn_sig.inputs
        .iter()
        .filter_map(|input| {
            match input {
                syn::FnArg::Typed(pat_type) => Some((*pat_type.ty).clone()),
                syn::FnArg::Receiver(receiver) => {
                    let self_ty = &impl_item.self_ty;
                    if receiver.reference.is_some() {
                        if receiver.mutability.is_some() {
                            Some(syn::parse_quote! { &mut #self_ty })
                        } else {
                            Some(syn::parse_quote! { &#self_ty })
                        }
                    } else {
                        Some(syn::parse_quote! { #self_ty })
                    }
                }
            }
        })
        .collect();

    let fn_output = &fn_sig.output;

    let replaced_self_fn_output = match fn_output {
        syn::ReturnType::Default => { syn::ReturnType::Default }
        syn::ReturnType::Type(arrow, ty) => {
            let mut ty = ty.clone();
            (ReplaceSelf {
                self_ty: impl_item.self_ty.as_ref(),
            }).visit_type_mut(ty.as_mut());
            syn::ReturnType::Type(*arrow, ty)
        }
    };

    let fn_ptr_tokens = quote! { fn(#(#fn_param_types),*) #replaced_self_fn_output };
    syn::parse(fn_ptr_tokens.into()).expect("Failed to parse function pointer type")
}

struct ReplaceSelf<'a> {
    self_ty: &'a syn::Type,
}

impl syn::visit_mut::VisitMut for ReplaceSelf<'_> {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        if let syn::Type::Path(syn::TypePath { qself: None, path }) = ty {
            if path.is_ident("Self") {
                *ty = self.self_ty.clone();
                return;
            }
        }

        syn::visit_mut::visit_type_mut(self, ty);
    }
}
