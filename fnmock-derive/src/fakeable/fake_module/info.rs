//! The information needed to generate a fake module.

use crate::{
    extract::{function::info::FunctionInfo, item_impl::info::ImplItemFnInfo},
    fakeable::fn_closure_trait::build_fn_closure_trait,
    names::{
        NameType, build_impl_interface_struct_name, build_impl_module_name, build_impl_store_name,
        build_interface_struct_name, build_module_name, build_store_name,
    },
};

/// Information needed to generate a fake module (the `thread_local` store + interface struct).
#[derive(Clone)]
pub struct FakeModuleInfo {
    /// The name of the generated module itself.
    pub module_name: syn::Ident,

    /// The name of the `thread_local` static holding the store.
    pub store_name: syn::Ident,

    /// How the faked function is referred to in panic messages, e.g. `"UserService get_user"`.
    pub display_name: String,

    /// The name of the interface struct carrying `setup`/`clear`/`is_set`/`get`.
    pub interface_struct_name: syn::Ident,

    pub visibility: syn::Visibility,

    /// The `Fn(..) -> ..` bound a fake must satisfy. Used both as the stored closure's type and as
    /// the bound on `setup`'s argument.
    pub fn_closure_trait: syn::TraitBound,

    /// The generic parameters to key the store by. `None` selects a plain `FakeStore`; `Some`
    /// selects a `GenericFakeStore`.
    pub generic_info: Option<FakeModuleGenericInfo>,
}

/// Information about the generic parameters for a fake module.
///
/// If it is a struct method fake, the generic parameters from the struct come first, followed by the generic parameters from the method.
#[derive(Clone)]
pub struct FakeModuleGenericInfo {
    /// The number of generic parameters, which becomes the `GENERIC_COUNT` const generic of the
    /// generated `GenericFakeStore`.
    pub generic_count: usize,

    /// The identifiers of the generic parameters.
    pub generic_idents: Vec<syn::Ident>,

    /// The generic parameters, including their bounds (e.g. `T: Display + 'static` and `I: 'static`).
    pub generic_params: Vec<syn::GenericParam>,

    /// The `GenericKeyPart` expressions for the generic parameters, in the order they appear in the code
    /// (e.g. `[GenericKeyPart::Type(TypeId::of::<T>()), GenericKeyPart::Const(I.into_const_value())]`).
    pub generic_keys: Vec<syn::Expr>,
}

impl TryFrom<&FunctionInfo> for FakeModuleInfo {
    type Error = syn::Error;

    fn try_from(function_info: &FunctionInfo) -> Result<Self, Self::Error> {
        let module_name = build_module_name(&function_info.name, NameType::Fake);
        let store_name = build_store_name(&function_info.name, NameType::Fake);
        let display_name = format!("{}", function_info.name);
        let interface_struct_name =
            build_interface_struct_name(&function_info.name, NameType::Fake);
        let fn_closure_trait = build_fn_closure_trait(
            &function_info.lifetimes,
            &function_info.param_types,
            &function_info.return_type,
            NameType::Fake,
        )?;

        Ok(FakeModuleInfo {
            module_name,
            store_name,
            display_name,
            interface_struct_name,
            visibility: function_info.visibility.clone(),
            fn_closure_trait,
            generic_info: function_info
                .generic_info
                .as_ref()
                .map(|info| FakeModuleGenericInfo {
                    generic_count: info.count,
                    generic_idents: info.idents.clone(),
                    generic_params: info.generic_params.clone(),
                    generic_keys: info.generic_keys.clone(),
                }),
        })
    }
}

impl TryFrom<&ImplItemFnInfo> for FakeModuleInfo {
    type Error = syn::Error;

    fn try_from(impl_item_fn_info: &ImplItemFnInfo) -> Result<Self, Self::Error> {
        let module_name = build_impl_module_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake,
        );
        let store_name = build_impl_store_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake,
        );
        let display_name = format!(
            "{} {}",
            impl_item_fn_info.struct_name, impl_item_fn_info.method_name
        );
        let interface_struct_name = build_impl_interface_struct_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake,
        );
        let fn_closure_trait = build_fn_closure_trait(
            &impl_item_fn_info.lifetimes,
            &impl_item_fn_info.param_types,
            &impl_item_fn_info.return_type,
            NameType::Fake,
        )?;

        Ok(FakeModuleInfo {
            module_name,
            store_name,
            display_name,
            interface_struct_name,
            visibility: impl_item_fn_info.visibility.clone(),
            fn_closure_trait,
            generic_info: impl_item_fn_info.generic_info.as_ref().map(|info| {
                FakeModuleGenericInfo {
                    generic_count: info.count,
                    generic_idents: info.idents.clone(),
                    generic_params: info.generic_params.clone(),
                    generic_keys: info.generic_keys.clone(),
                }
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::function::extract_function_info;
    use crate::extract::item_impl::extract_item_impl_info;
    use quote::ToTokens;

    fn render_idents(idents: &[syn::Ident]) -> Vec<String> {
        idents
            .iter()
            .map(|ident| ident.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_try_from_function_info_standalone_function() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn get_user(id: u32) -> String {
                todo!()
            }
        };
        let function_info =
            extract_function_info(&item_fn, NameType::Fake).expect("valid standalone function");

        let info = FakeModuleInfo::try_from(&function_info)
            .expect("conversion should succeed for a non-generic standalone function");

        assert_eq!(info.module_name.to_string(), "get_user_fake_module");
        assert_eq!(info.store_name.to_string(), "GET_USER_FAKE_STORE");
        assert_eq!(info.display_name, "get_user");
        assert_eq!(
            info.interface_struct_name.to_string(),
            "GetUserFakeInterface"
        );
        assert!(
            info.generic_info.is_none(),
            "expected no generic_info for a non-generic standalone function"
        );
    }

    #[test]
    fn test_try_from_impl_item_fn_info_method() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl UserService {
                fn get_user(&self, id: u32) -> String {
                    todo!()
                }
            }
        };
        let impl_infos = extract_item_impl_info(&item_impl).expect("valid inherent impl block");
        let impl_info = &impl_infos[0];

        let info = FakeModuleInfo::try_from(impl_info)
            .expect("conversion should succeed for a non-generic impl method");

        assert_eq!(
            info.module_name.to_string(),
            "user_service__get_user_fake_module"
        );
        assert_eq!(
            info.store_name.to_string(),
            "USER_SERVICE_GET_USER_FAKE_STORE"
        );
        assert_eq!(info.display_name, "UserService get_user");
        assert_eq!(
            info.interface_struct_name.to_string(),
            "UserServiceGetUserFakeInterface"
        );
    }

    #[test]
    fn test_try_from_function_info_generic_function_maps_generic_info() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn foo<T>(x: T) -> T {
                x
            }
        };
        let function_info = extract_function_info(&item_fn, NameType::Fake)
            .expect("valid generic standalone function");

        let info = FakeModuleInfo::try_from(&function_info)
            .expect("conversion should succeed for a generic standalone function");

        let Some(generic_info) = info.generic_info else {
            panic!("expected generic_info to be Some for a generic standalone function");
        };
        assert_eq!(generic_info.generic_count, 1);
        assert_eq!(
            render_idents(&generic_info.generic_idents),
            vec!["T".to_string()]
        );
    }
}
