use crate::{
    extract::{function::info::FunctionInfo, item_impl::info::ImplItemFnInfo},
    names::{
        build_access_function_name, build_impl_interface_struct_name, build_impl_module_name,
        build_interface_struct_name, build_module_name, NameType,
    },
};

/// Information needed to generate an access function/method for a fake (e.g. `get_user_fake()`).
#[derive(Clone)]
pub struct AccessFunctionInfo {
    /// Access function name for the fake interface (e.g. "get_user_fake").
    pub access_function_name: syn::Ident,

    /// The name of the fake module this access function reaches into.
    pub module_name: syn::Ident,

    /// The name of the struct that provides the API for setting up and accessing the fake implementation.
    pub interface_struct_name: syn::Ident,

    pub generic_info: Option<AccessFunctionGenericInfo>,
}

/// Information about the generic parameters needed to declare the access function/method itself.
///
/// For a standalone function, `generic_params` is the function's own (bounded) generic
/// parameters. For an impl block method, `generic_params` is only the method's own generic
/// parameters — the struct's generic parameters are already in scope from the enclosing
/// `impl<...>` block. In both cases `generic_idents` is the full list of identifiers (struct +
/// method, where applicable) needed to instantiate the fake module's interface struct.
#[derive(Clone)]
pub struct AccessFunctionGenericInfo {
    pub generic_idents: Vec<syn::Ident>,
    pub generic_params: Vec<syn::GenericParam>,
}

impl TryFrom<&FunctionInfo> for AccessFunctionInfo {
    type Error = syn::Error;

    fn try_from(function_info: &FunctionInfo) -> Result<Self, Self::Error> {
        let access_function_name = build_access_function_name(&function_info.name, NameType::Fake);
        let module_name = build_module_name(&function_info.name, NameType::Fake);
        let interface_struct_name =
            build_interface_struct_name(&function_info.name, NameType::Fake);

        Ok(AccessFunctionInfo {
            access_function_name,
            module_name,
            interface_struct_name,
            generic_info: function_info.generic_info.as_ref().map(|info| {
                AccessFunctionGenericInfo {
                    generic_idents: info.idents.clone(),
                    generic_params: info.generic_params.clone(),
                }
            }),
        })
    }
}

impl TryFrom<&ImplItemFnInfo> for AccessFunctionInfo {
    type Error = syn::Error;

    fn try_from(impl_item_fn_info: &ImplItemFnInfo) -> Result<Self, Self::Error> {
        let access_function_name =
            build_access_function_name(&impl_item_fn_info.method_name, NameType::Fake);
        let module_name = build_impl_module_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake,
        );
        let interface_struct_name = build_impl_interface_struct_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake,
        );

        Ok(AccessFunctionInfo {
            access_function_name,
            module_name,
            interface_struct_name,
            generic_info: impl_item_fn_info.generic_info.as_ref().map(|info| {
                AccessFunctionGenericInfo {
                    generic_idents: info.idents.clone(),
                    generic_params: info.method_generic_params.clone(),
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

    fn render_params(params: &[syn::GenericParam]) -> Vec<String> {
        params
            .iter()
            .map(|param| param.to_token_stream().to_string())
            .collect()
    }

    #[test]
    fn test_try_from_function_info_standalone_function() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn get_user(id: u32) -> String {
                todo!()
            }
        };
        let function_info = extract_function_info(&item_fn).expect("valid standalone function");

        let info = AccessFunctionInfo::try_from(&function_info)
            .expect("conversion should succeed for a non-generic standalone function");

        assert_eq!(info.access_function_name.to_string(), "get_user_fake");
        assert_eq!(info.module_name.to_string(), "get_user_fake_module");
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

        let info = AccessFunctionInfo::try_from(impl_info)
            .expect("conversion should succeed for a non-generic impl method");

        assert_eq!(info.access_function_name.to_string(), "get_user_fake");
        assert_eq!(
            info.module_name.to_string(),
            "user_service_struct_get_user_fake_module"
        );
        assert_eq!(
            info.interface_struct_name.to_string(),
            "UserServiceGetUserFakeInterface"
        );
    }

    #[test]
    fn test_try_from_impl_item_fn_info_generic_struct_and_method_combines_types_but_keeps_params_method_only(
    ) {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl<S> Foo<S> {
                fn bar<M>(&self, x: M) -> S {
                    todo!()
                }
            }
        };
        let impl_infos = extract_item_impl_info(&item_impl)
            .expect("valid inherent impl block with struct and method generics");
        let impl_info = &impl_infos[0];

        let info = AccessFunctionInfo::try_from(impl_info)
            .expect("conversion should succeed for a generic impl method");

        let Some(generic_info) = info.generic_info else {
            panic!("expected generic_info to be Some when the struct and method are generic");
        };
        // Combined struct + method types are needed to instantiate the fake module's interface struct.
        assert_eq!(
            render_idents(&generic_info.generic_idents),
            vec!["S".to_string(), "M".to_string()]
        );
        // Only the method's own generic params should be re-declared by the access method; the
        // struct's params are already in scope from the enclosing `impl<...>` block.
        assert_eq!(
            render_params(&generic_info.generic_params),
            vec!["M".to_string()]
        );
    }

    #[test]
    fn test_try_from_function_info_generic_function_declares_its_own_full_generic_list() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn foo<T>(x: T) -> T {
                x
            }
        };
        let function_info =
            extract_function_info(&item_fn).expect("valid generic standalone function");

        let info = AccessFunctionInfo::try_from(&function_info)
            .expect("conversion should succeed for a generic standalone function");

        let Some(generic_info) = info.generic_info else {
            panic!("expected generic_info to be Some for a generic standalone function");
        };
        assert_eq!(
            render_params(&generic_info.generic_params),
            vec!["T".to_string()]
        );
    }
}
