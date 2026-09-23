use crate::{
    item_info::function::FunctionInfo,
    scheme::{
        common::{function::FunctionCommonScheme, generic_scheme::build_generic_scheme},
        fake::{FakeScheme, function::build_fake_scheme},
        mock::function::names::{build_accessor_name, build_interface_name, build_module_name},
        spy::{SpyScheme, function::build_spy_scheme},
    },
};

mod names;

pub struct FunctionMockScheme {
    pub common: FunctionCommonScheme,

    pub fake: FakeScheme,
    pub spy: SpyScheme,
}

impl TryFrom<FunctionInfo> for FunctionMockScheme {
    type Error = syn::Error;

    fn try_from(value: FunctionInfo) -> Result<Self, Self::Error> {
        let fake = build_fake_scheme(&value)?;
        let spy = build_spy_scheme(&value)?;

        let module_name = build_module_name(&value.name);
        let accessor_name = build_accessor_name(&value.name);
        let interface_name = build_interface_name(&value.name);
        let display_name = value.name.to_string();
        let generic_scheme = build_generic_scheme(&value.generic_params);

        Ok(FunctionMockScheme {
            common: FunctionCommonScheme {
                vis: value.visibility,
                original: value.original,
                module_name,
                display_name,
                accessor_name,
                interface_name,
                generic_scheme,
            },
            fake,
            spy,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_standalone_function_carries_both_halves() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn get_user(id: u32) -> String {
                todo!()
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionMockScheme::try_from(info).expect("conversion should succeed");

        assert_eq!(
            scheme.common.module_name.to_string(),
            "get_user_mock_module"
        );
        assert_eq!(scheme.common.accessor_name.to_string(), "get_user_mock");
        assert_eq!(
            scheme.common.interface_name.to_string(),
            "GetUserMockInterface"
        );
        assert_eq!(scheme.fake.store_name.to_string(), "GET_USER_FAKE_STORE");
        assert_eq!(scheme.spy.store_name.to_string(), "GET_USER_SPY_STORE");
    }

    #[test]
    fn test_return_impl_trait_is_rejected_by_the_fake_half() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo() -> impl Clone {
                1
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        assert!(FunctionMockScheme::try_from(info).is_err());
    }

    #[test]
    fn test_destructuring_param_is_rejected_by_the_spy_half() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo((a, b): (i32, i32)) {}
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let Err(error) = FunctionMockScheme::try_from(info) else {
            panic!("a destructuring parameter should be rejected");
        };
        assert!(error.to_string().contains("destructures its value"));
    }
}
