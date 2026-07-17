use syn::{ spanned::Spanned, visit_mut::VisitMut };

use crate::extract::{
    fn_closure_trait::build_fn_closure_trait,
    item_impl::{ generics::extract_generic_impl_info, info::ImplItemFnInfo },
    lifetimes::extract_lifetimes_from_generics,
    params::{ extract_param_pats, extract_param_types },
    replace_self::ReplaceSelf,
};

pub mod info;
mod generics;

/// Extract the ImplItemFnInfo for each method in an impl block.
pub fn extract_item_impl_info(item_impl: &syn::ItemImpl) -> syn::Result<Vec<ImplItemFnInfo>> {
    if let Some((_, trait_path, _)) = &item_impl.trait_ {
        return Err(
            syn::Error::new_spanned(
                trait_path,
                "The #[fakeable] attribute does not support trait impl blocks (`impl Trait for Type`). Only inherent impl blocks (`impl Type { ... }`) are supported."
            )
        );
    }

    let mut method_infos = Vec::new();

    for item in &item_impl.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_info = extract_single_item_impl_info_for_method(item_impl, method)?;
            method_infos.push(method_info);
        }
    }

    Ok(method_infos)
}

/// Extract the ImplItemFnInfo for a single method in an impl block.
fn extract_single_item_impl_info_for_method(
    item_impl: &syn::ItemImpl,
    method: &syn::ImplItemFn
) -> syn::Result<ImplItemFnInfo> {
    if let Some(const_token) = &method.sig.constness {
        return Err(
            syn::Error::new_spanned(
                const_token,
                "The #[fakeable] attribute does not support const fn. The fake lookup fnmock injects cannot run in a const context."
            )
        );
    }

    let struct_name = extract_struct_ident(&item_impl.self_ty)?;
    let method_name = method.sig.ident.clone();

    let generic_info = extract_generic_impl_info(item_impl, method)?;
    let struct_lifetimes = extract_lifetimes_from_generics(&item_impl.generics);
    let method_lifetimes = extract_lifetimes_from_generics(&method.sig.generics);
    // We know, there can be no duplicate lifetimes between the struct and method, because Rust would not allow that in the first place.
    // Therefore, we can safely combine the lifetimes from the struct and method into a single list of lifetimes for the function pointer type.
    let lifetimes = struct_lifetimes
        .clone()
        .into_iter()
        .chain(method_lifetimes.clone().into_iter())
        .collect::<Vec<_>>();

    let params = method.sig.inputs.iter().cloned().collect::<Vec<_>>();
    let param_types = extract_param_types(&params, Some(&item_impl.self_ty))?;
    let param_pats = extract_param_pats(&params);

    let return_type = extract_return_type(&method.sig.output, &item_impl.self_ty);

    let fn_closure_trait = build_fn_closure_trait(&lifetimes, &param_types, &return_type)?;

    Ok(ImplItemFnInfo {
        struct_name,
        method_name,
        _param_types: param_types,
        param_pats,
        fn_closure_trait,
        generic_info,
    })
}

/// Extract the struct identifier from the `self_ty` of an impl block.
fn extract_struct_ident(self_ty: &syn::Type) -> syn::Result<syn::Ident> {
    match self_ty {
        syn::Type::Path(tp) => {
            // Usually the last segment is the concrete type.
            // Example: Foo<T> -> Path segments [..., Foo<T>]
            let seg = tp.path.segments.last().ok_or_else(||
                syn::Error::new(
                    self_ty.span(),
                    "internal error: expected the impl type path to have at least one segment. This is a bug in fnmock; please report it."
                )
            )?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_impl_block_is_rejected() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl SomeTrait for SomeStruct {
                fn method(&self) -> i32 { 42 }
            }
        };

        let result = extract_item_impl_info(&item_impl);

        assert!(result.is_err(), "expected #[fakeable] on a trait impl block to be rejected");
    }

    #[test]
    fn test_inherent_impl_block_is_accepted() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl SomeStruct {
                fn method(&self) -> i32 { 42 }
            }
        };

        let result = extract_item_impl_info(&item_impl);

        assert!(result.is_ok(), "expected #[fakeable] on an inherent impl block to be accepted");
    }

    #[test]
    fn test_const_method_is_rejected() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl SomeStruct {
                const fn method(a: i32) -> i32 { a }
            }
        };

        let result = extract_item_impl_info(&item_impl);

        assert!(result.is_err(), "expected #[fakeable] on a const method to be rejected");
    }
}
