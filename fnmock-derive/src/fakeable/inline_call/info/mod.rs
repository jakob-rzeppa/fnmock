//! The information needed to generate the injected fake lookup.

use crate::{
    extract::{function::info::FunctionInfo, item_impl::info::ImplItemFnInfo},
    fakeable::inline_call::info::fake_call_value::FakeCallValue,
    names::{
        build_impl_interface_struct_name, build_impl_module_name, build_interface_struct_name,
        build_module_name, NameType,
    },
};

mod fake_call_value;

/// Everything the injected fake lookup needs to name the fake and forward the call to it.
#[derive(Clone)]
pub struct InlineCallInfo {
    /// The name of the fake module the lookup reaches into.
    pub module_name: syn::Ident,

    /// The name of the interface struct the lookup asks for the fake.
    pub interface_struct_name: syn::Ident,

    /// The expressions forwarded as arguments to the fake, derived from the function's parameter
    /// patterns and in the same order.
    pub fake_call_values: Vec<FakeCallValue>,

    /// The generic parameters to instantiate the interface struct with, or `None` for a
    /// non-generic function.
    pub generic_idents: Option<Vec<syn::Ident>>,
}

impl TryFrom<&FunctionInfo> for InlineCallInfo {
    type Error = syn::Error;

    fn try_from(function_info: &FunctionInfo) -> Result<Self, Self::Error> {
        let module_name = build_module_name(&function_info.name, NameType::Fake);
        let interface_struct_name =
            build_interface_struct_name(&function_info.name, NameType::Fake);

        let fake_call_values = function_info
            .param_pats
            .iter()
            .map(FakeCallValue::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let generic_idents = function_info
            .generic_info
            .as_ref()
            .map(|generic_info| generic_info.idents.clone());

        Ok(InlineCallInfo {
            module_name,
            interface_struct_name,
            fake_call_values,
            generic_idents,
        })
    }
}

impl TryFrom<&ImplItemFnInfo> for InlineCallInfo {
    type Error = syn::Error;

    fn try_from(impl_item_fn_info: &ImplItemFnInfo) -> Result<Self, Self::Error> {
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

        let fake_call_values = impl_item_fn_info
            .param_pats
            .iter()
            .map(FakeCallValue::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let generic_idents = impl_item_fn_info
            .generic_info
            .as_ref()
            .map(|generic_info| generic_info.idents.clone());

        Ok(InlineCallInfo {
            module_name,
            interface_struct_name,
            fake_call_values,
            generic_idents,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::function::extract_function_info;
    use crate::extract::item_impl::extract_item_impl_info;
    use quote::ToTokens;

    fn render_fake_call_values(values: &[FakeCallValue]) -> Vec<String> {
        values
            .iter()
            .map(|value| value.to_token_stream().to_string())
            .collect()
    }

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
                unimplemented!()
            }
        };
        let function_info = extract_function_info(&item_fn).expect("valid standalone function");

        let info = InlineCallInfo::try_from(&function_info)
            .expect("conversion should succeed for a non-generic standalone function");

        assert_eq!(info.module_name.to_string(), "get_user_fake_module");
        assert_eq!(
            info.interface_struct_name.to_string(),
            "GetUserFakeInterface"
        );
        assert_eq!(
            render_fake_call_values(&info.fake_call_values),
            vec!["id".to_string()]
        );
        assert!(
            info.generic_idents.is_none(),
            "expected no generic_idents for a non-generic standalone function"
        );
    }

    #[test]
    fn test_try_from_function_info_multiple_params_preserves_order() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn add(a: i32, b: i32) -> i32 {
                todo!()
            }
        };
        let function_info = extract_function_info(&item_fn).expect("valid standalone function");

        let info = InlineCallInfo::try_from(&function_info).expect("conversion should succeed");

        assert_eq!(
            render_fake_call_values(&info.fake_call_values),
            vec!["a".to_string(), "b".to_string()]
        );
    }

    #[test]
    fn test_try_from_function_info_tuple_pattern_param_becomes_tuple_fake_call_value() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn foo((a, b): (i32, i32)) {}
        };
        let function_info = extract_function_info(&item_fn).expect("valid standalone function");

        let info = InlineCallInfo::try_from(&function_info).expect("conversion should succeed");

        assert_eq!(info.fake_call_values.len(), 1);
        assert_eq!(
            info.fake_call_values[0].to_token_stream().to_string(),
            quote::quote!((a, b)).to_string()
        );
    }

    #[test]
    fn test_try_from_function_info_generic_function_carries_generic_types() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn foo<T>(x: T) -> T {
                x
            }
        };
        let function_info =
            extract_function_info(&item_fn).expect("valid generic standalone function");

        let info = InlineCallInfo::try_from(&function_info)
            .expect("conversion should succeed for a generic standalone function");

        let Some(generic_idents) = info.generic_idents else {
            panic!("expected generic_idents to be Some for a generic standalone function");
        };
        assert_eq!(render_idents(&generic_idents), vec!["T".to_string()]);
    }

    #[test]
    fn test_try_from_function_info_propagates_error_from_unsupported_param_pattern() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn foo(ref x: i32) {}
        };
        let function_info = extract_function_info(&item_fn).expect("valid standalone function");

        let result = InlineCallInfo::try_from(&function_info);

        let Err(error) = result else {
            panic!("a `ref` parameter pattern should be rejected during conversion");
        };
        let message = error.to_string().to_lowercase();
        assert!(
            message.contains("ref"),
            "error message should mention `ref`, got: {message}"
        );
    }

    #[test]
    fn test_try_from_impl_item_fn_info_method_includes_self_and_params() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl UserService {
                fn get_user(&self, id: u32) -> String {
                    todo!()
                }
            }
        };
        let impl_infos = extract_item_impl_info(&item_impl).expect("valid inherent impl block");
        let impl_info = &impl_infos[0];

        let info = InlineCallInfo::try_from(impl_info)
            .expect("conversion should succeed for a non-generic impl method");

        assert_eq!(
            info.module_name.to_string(),
            "user_service__get_user_fake_module"
        );
        assert_eq!(
            info.interface_struct_name.to_string(),
            "UserServiceGetUserFakeInterface"
        );
        assert_eq!(
            render_fake_call_values(&info.fake_call_values),
            vec!["self".to_string(), "id".to_string()]
        );
        assert!(
            info.generic_idents.is_none(),
            "expected no generic_idents for a non-generic impl method"
        );
    }

    #[test]
    fn test_try_from_impl_item_fn_info_generic_struct_and_method_combines_types() {
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

        let info = InlineCallInfo::try_from(impl_info)
            .expect("conversion should succeed for a generic impl method");

        let Some(generic_idents) = info.generic_idents else {
            panic!("expected generic_idents to be Some when the struct and method are generic");
        };
        assert_eq!(
            render_idents(&generic_idents),
            vec!["S".to_string(), "M".to_string()]
        );
    }

    #[test]
    fn test_try_from_impl_item_fn_info_propagates_error_from_unsupported_param_pattern() {
        let item_impl: syn::ItemImpl = syn::parse_quote! {
            impl UserService {
                fn get_user(ref x: i32) {}
            }
        };
        let impl_infos = extract_item_impl_info(&item_impl).expect("valid inherent impl block");
        let impl_info = &impl_infos[0];

        let result = InlineCallInfo::try_from(impl_info);

        let Err(error) = result else {
            panic!("a `ref` parameter pattern should be rejected during conversion");
        };
        let message = error.to_string().to_lowercase();
        assert!(
            message.contains("ref"),
            "error message should mention `ref`, got: {message}"
        );
    }
}
