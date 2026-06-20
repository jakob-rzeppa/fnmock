use syn::{ spanned::Spanned, visit_mut::VisitMut };

use crate::extract::{
    fn_ptr_type::build_fn_ptr_type,
    generic::extract_generic_impl_info,
    params::{ extract_param_idents, extract_param_types },
    replace_self::ReplaceSelf,
};

pub struct ItemImplMethodInfo {
    pub struct_name: syn::Ident,
    pub method_name: syn::Ident,
    pub param_types: Vec<syn::Type>,
    pub param_idents: Vec<syn::Ident>,
    pub fn_ptr_type: syn::Type,
    pub generic_info: Option<ItemImplMethodGenericInfo>,
}

/// The generics of the struct and method are combined, in the order of struct generics followed by method generics.
pub struct ItemImplMethodGenericInfo {
    pub count: usize,
    pub type_params: Vec<syn::TypeParam>,
    pub idents: Vec<syn::Ident>,
    pub type_ids: Vec<syn::Expr>,
}

/// Extract the ItemImplMethodInfo for each method in an impl block.
pub fn extract_item_impl_info(item_impl: &syn::ItemImpl) -> syn::Result<Vec<ItemImplMethodInfo>> {
    let mut method_infos = Vec::new();

    for item in &item_impl.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_info = extract_single_item_impl_info_for_method(item_impl, method)?;
            method_infos.push(method_info);
        }
    }

    Ok(method_infos)
}

/// Extract the ItemImplMethodInfo for a single method in an impl block.
fn extract_single_item_impl_info_for_method(
    item_impl: &syn::ItemImpl,
    method: &syn::ImplItemFn
) -> syn::Result<ItemImplMethodInfo> {
    let struct_name = extract_struct_ident(&item_impl.self_ty)?;
    let method_name = method.sig.ident.clone();

    let params = method.sig.inputs.iter().cloned().collect::<Vec<_>>();
    let param_types = extract_param_types(&params, Some(&item_impl.self_ty));
    let param_idents = extract_param_idents(&params);

    let return_type = extract_return_type(&method.sig.output, &item_impl.self_ty);
    let fn_ptr_type = build_fn_ptr_type(&param_types, &return_type)?;

    let generic_info = extract_generic_impl_info(item_impl, method);

    Ok(ItemImplMethodInfo {
        struct_name,
        method_name,
        param_types,
        param_idents,
        fn_ptr_type,
        generic_info: generic_info.map(|info| ItemImplMethodGenericInfo {
            count: info.generic_count,
            type_params: info.generic_type_params,
            idents: info.generic_idents,
            type_ids: info.generic_type_ids,
        }),
    })
}

/// Extract the struct identifier from the `self_ty` of an impl block.
fn extract_struct_ident(self_ty: &syn::Type) -> syn::Result<syn::Ident> {
    match self_ty {
        syn::Type::Path(tp) => {
            // Usually the last segment is the concrete type.
            // Example: Foo<T> -> Path segments [..., Foo<T>]
            let seg = tp.path.segments.last().expect("Expected at least one segment in path");
            Ok(seg.ident.clone())
        }
        _ => {
            Err(
                syn::Error::new(
                    self_ty.span(),
                    "Unsupported struct type. Only simple paths (+generics) are supported for impl blocks."
                )
            )
        }
    }
}

/// Extract the return type from a method, replacing any `Self` types with the actual type of `Self` from the impl block.
fn extract_return_type(output: &syn::ReturnType, self_ty: &syn::Type) -> syn::ReturnType {
    let mut self_replacer = ReplaceSelf::new(self_ty);

    match output {
        syn::ReturnType::Default => { syn::ReturnType::Default }
        syn::ReturnType::Type(arrow, ty) => {
            let mut ty = ty.clone();
            self_replacer.visit_type_mut(ty.as_mut());
            syn::ReturnType::Type(*arrow, ty)
        }
    }
}
