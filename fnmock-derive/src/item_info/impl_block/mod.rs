//! The information extracted from an inherent impl block and its methods.

use syn::spanned::Spanned;

use crate::item_info::{
    generic_param_info::{GenericParamInfo, extract_generic_param_infos},
    lifetimes::extract_lifetimes_from_generics,
    original::OriginalImpl,
    param_info::{ParamInfo, extract_params},
};

/// Everything shared across every method of one fakeable/spyable inherent impl block, plus one
/// [`ImplItemFnInfo`] per method.
pub struct ImplBlockInfo {
    pub original: OriginalImpl,

    /// The type the impl block is for, kept as the full path (module segments + generic
    /// arguments) rather than truncated to its last segment. Combined with each method's name to
    /// keep the generated names of two same-named methods on different types apart — the
    /// name-building functions in `scheme::impl_block::fake::names` are what mangle this into an
    /// identifier, so the full path must survive until then.
    pub struct_name: syn::TypePath,

    /// The struct's generic parameters, in declaration order.
    pub generic_param_infos: Vec<GenericParamInfo>,

    /// One entry per method in the impl block, in the order the methods appear in the block.
    pub functions: Vec<ImplMethodInfo>,
}

/// Everything the generators need to know about one fakeable/spyable method of an inherent impl
/// block, beyond what's shared with its sibling methods on [`ItemImplInfo`].
pub struct ImplMethodInfo {
    /// The method's own name.
    pub method_name: syn::Ident,

    /// The method's visibility, which is copied to the generated items.
    pub visibility: syn::Visibility,

    /// The parameters, in declaration order, with the receiver represented as a plain `self`
    /// pattern and every `Self` type replaced by the impl block's concrete type. The receiver is
    /// included as the first entry. Used to forward the call's arguments to the fake closure and
    /// to build the `Fn(..) -> ..` trait bound a fake must satisfy.
    pub param_infos: Vec<ParamInfo>,

    /// The combined lifetime parameters of the struct and method. Only a fake needs these, to
    /// bind them higher-ranked on its closure trait.
    pub lifetimes: Vec<syn::Lifetime>,

    /// The method's return type, with every `Self` replaced by the impl block's concrete type.
    pub return_type: syn::ReturnType,

    /// The method's generic parameters, in declaration order.
    pub generic_param_infos: Vec<GenericParamInfo>,
}

impl TryFrom<syn::ItemImpl> for ImplBlockInfo {
    type Error = syn::Error;

    fn try_from(item_impl: syn::ItemImpl) -> Result<Self, Self::Error> {
        if let Some((_, trait_path, _)) = item_impl.trait_ {
            return Err(syn::Error::new_spanned(
                trait_path,
                "The macro does not support trait impl blocks (`impl Trait for Type`). Only inherent impl blocks (`impl Type {{ ... }}`) are supported.",
            ));
        }

        let struct_name = extract_struct_path(&item_impl.self_ty)?;
        let generic_param_infos = extract_generic_param_infos(&item_impl.generics)?;

        let mut functions = Vec::new();
        for item in &item_impl.items {
            if let syn::ImplItem::Fn(method) = item {
                let method_info = extract_single_item_impl_info_for_method(&item_impl, method)?;
                functions.push(method_info);
            }
        }

        Ok(ImplBlockInfo {
            original: OriginalImpl::new(item_impl),
            struct_name,
            generic_param_infos,
            functions,
        })
    }
}

/// Extract the [`ImplMethodInfo`] for a single method in an impl block.
fn extract_single_item_impl_info_for_method(
    item_impl: &syn::ItemImpl,
    method: &syn::ImplItemFn,
) -> syn::Result<ImplMethodInfo> {
    if let Some(const_token) = &method.sig.constness {
        return Err(syn::Error::new_spanned(
            const_token,
            "The macro does not support const fn. The code fnmock injects cannot run in a const context.".to_string(),
        ));
    }

    let method_name = method.sig.ident.clone();
    let visibility = method.vis.clone();

    let generic_param_infos = extract_generic_param_infos(&method.sig.generics)?;
    let struct_lifetimes = extract_lifetimes_from_generics(&item_impl.generics);
    let method_lifetimes = extract_lifetimes_from_generics(&method.sig.generics);
    // We know, there can be no duplicate lifetimes between the struct and method, because Rust would not allow that in the first place.
    // Therefore, we can safely combine the lifetimes from the struct and method into a single list of lifetimes for the function pointer type.
    let lifetimes = struct_lifetimes
        .into_iter()
        .chain(method_lifetimes)
        .collect::<Vec<_>>();

    let fn_args = method.sig.inputs.iter().cloned().collect::<Vec<_>>();
    let params = extract_params(&fn_args, Some(&item_impl.self_ty))?;

    let return_type = extract_return_type(&method.sig.output, &item_impl.self_ty);

    Ok(ImplMethodInfo {
        method_name,
        visibility,
        param_infos: params,
        lifetimes,
        return_type,
        generic_param_infos,
    })
}

/// Extract the full type path from the `self_ty` of an impl block.
fn extract_struct_path(self_ty: &syn::Type) -> syn::Result<syn::TypePath> {
    match self_ty {
        syn::Type::Path(tp) => Ok(tp.clone()),
        _ => Err(syn::Error::new(
            self_ty.span(),
            "Unsupported struct type. Only simple paths (+generics) are supported for impl blocks.",
        )),
    }
}

/// Extract the return type from a method, replacing any `Self` types with the actual type of `Self` from the impl block.
fn extract_return_type(output: &syn::ReturnType, self_ty: &syn::Type) -> syn::ReturnType {
    use syn::visit_mut::VisitMut;

    let mut self_replacer = crate::item_info::replace_self::ReplaceSelf::new(self_ty);

    match output {
        syn::ReturnType::Default => syn::ReturnType::Default,
        syn::ReturnType::Type(arrow, ty) => {
            let mut ty = ty.clone();
            self_replacer.visit_type_mut(ty.as_mut());
            syn::ReturnType::Type(*arrow, ty)
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    #[test]
    fn test_trait_impl_block_is_rejected() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl SomeTrait for SomeStruct {
                fn method(&self) -> i32 { 42 }
            }
        };

        let result = ImplBlockInfo::try_from(item_impl);

        assert!(
            result.is_err(),
            "expected #[fakeable] on a trait impl block to be rejected"
        );
    }

    #[test]
    fn test_inherent_impl_block_is_accepted() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl SomeStruct {
                fn method(&self) -> i32 { 42 }
            }
        };

        let result = ImplBlockInfo::try_from(item_impl);

        assert!(
            result.is_ok(),
            "expected #[fakeable] on an inherent impl block to be accepted"
        );
    }

    #[test]
    fn test_const_method_is_rejected() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl SomeStruct {
                const fn method(a: i32) -> i32 { a }
            }
        };

        let result = ImplBlockInfo::try_from(item_impl);

        assert!(
            result.is_err(),
            "expected #[fakeable] on a const method to be rejected"
        );
    }

    #[test]
    fn test_struct_name_lives_on_item_impl_info_not_per_method() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl UserService {
                fn get_user(&self) -> i32 { 42 }
                fn save_user(&self) {}
            }
        };

        let info = ImplBlockInfo::try_from(item_impl).expect("valid inherent impl block");

        assert_eq!(
            info.struct_name.to_token_stream().to_string(),
            quote::quote!(UserService).to_string()
        );
        assert_eq!(info.functions.len(), 2);
        assert_eq!(info.functions[0].method_name.to_string(), "get_user");
        assert_eq!(info.functions[1].method_name.to_string(), "save_user");
    }

    #[test]
    fn test_struct_name_captures_full_module_path() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl a::Config {
                fn basic(&self) -> i32 { 1 }
            }
        };

        let info = ImplBlockInfo::try_from(item_impl).expect("valid inherent impl block");

        assert_eq!(
            info.struct_name.to_token_stream().to_string(),
            quote::quote!(a::Config).to_string()
        );
    }

    #[test]
    fn test_struct_name_differs_for_same_type_name_in_different_modules() {
        let impl_a: syn::ItemImpl = syn::parse_quote! {
            impl a::Config {
                fn basic(&self) -> i32 { 1 }
            }
        };
        let impl_b: syn::ItemImpl = syn::parse_quote! {
            impl b::Config {
                fn basic(&self) -> i32 { 2 }
            }
        };

        let info_a = ImplBlockInfo::try_from(impl_a).expect("valid inherent impl block");
        let info_b = ImplBlockInfo::try_from(impl_b).expect("valid inherent impl block");

        assert_ne!(
            info_a.struct_name.to_token_stream().to_string(),
            info_b.struct_name.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_struct_generics_are_extracted_once_not_once_per_method() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl<S> Foo<S> {
                fn bar(&self, x: S) {}
                fn baz(&self, x: S) {}
            }
        };

        let info = ImplBlockInfo::try_from(item_impl).expect("valid generic impl block");

        assert_eq!(
            info.generic_param_infos
                .iter()
                .map(|i| i.ident.to_string())
                .collect::<Vec<_>>(),
            vec!["S".to_string()]
        );
        assert_eq!(info.functions.len(), 2);
    }

    #[test]
    fn test_method_only_generics_do_not_populate_struct_generic_param_infos() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl Foo {
                fn bar<M>(&self, x: M) {}
            }
        };

        let info =
            ImplBlockInfo::try_from(item_impl).expect("valid impl block with a generic method");

        assert!(
            info.generic_param_infos.is_empty(),
            "expected the struct's generic_param_infos to be empty when only the method is generic"
        );
        assert_eq!(
            info.functions[0]
                .generic_param_infos
                .iter()
                .map(|i| i.ident.to_string())
                .collect::<Vec<_>>(),
            vec!["M".to_string()],
            "expected the method's own generic_param_infos to contain M"
        );
    }
}
