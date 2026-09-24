use crate::{
    item_info::{
        generics::GenericParamInfo,
        impl_block::{ImplBlockInfo, ImplMethodInfo},
    },
    scheme::{
        common::{
            generic_scheme::build_generic_scheme,
            impl_block::{ImplCommonMethodScheme, ImplCommonScheme, combine_generic_param_infos},
        },
        fake::{FakeScheme, impl_block::build_fake_scheme},
        mock::impl_block::names::{build_accessor_name, build_interface_name, build_module_name},
        spy::{SpyScheme, impl_block::build_spy_scheme},
    },
};

mod names;

pub struct ImplMockScheme {
    pub common: ImplCommonScheme,

    /// The order of the methods must be preserved from the original impl block.
    /// (method_name, method_info)
    pub methods: Vec<(syn::Ident, ImplMockMethodScheme)>,
}

pub struct ImplMockMethodScheme {
    pub common: ImplCommonMethodScheme,

    pub fake: FakeScheme,
    pub spy: SpyScheme,
}

impl TryFrom<ImplBlockInfo> for ImplMockScheme {
    type Error = syn::Error;

    fn try_from(value: ImplBlockInfo) -> Result<Self, Self::Error> {
        let ImplBlockInfo {
            original,
            struct_name,
            generic_param_infos,
            functions,
        } = value;

        let methods = functions
            .into_iter()
            .map(|method| build_method_scheme(&struct_name, &generic_param_infos, method))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ImplMockScheme {
            common: ImplCommonScheme { original },
            methods,
        })
    }
}

/// Builds the mock scheme for a single method, merging the struct's generics (shared by every
/// method) with the method's own.
fn build_method_scheme(
    struct_name: &syn::TypePath,
    struct_generic_param_infos: &[GenericParamInfo],
    method: ImplMethodInfo,
) -> syn::Result<(syn::Ident, ImplMockMethodScheme)> {
    let fake = build_fake_scheme(struct_name, &method)?;

    let ImplMethodInfo {
        method_name,
        visibility,
        param_infos,
        lifetimes: _,
        return_type: _,
        generic_param_infos: method_generic_param_infos,
    } = method;

    let (method_generic_params, combined_generic_param_infos) =
        combine_generic_param_infos(struct_generic_param_infos, method_generic_param_infos);

    let spy = build_spy_scheme(
        struct_name,
        &method_name,
        &param_infos,
        &combined_generic_param_infos,
    )?;

    let module_name = build_module_name(struct_name, &method_name);
    let accessor_name = build_accessor_name(&method_name);
    let interface_name = build_interface_name(struct_name, &method_name)?;
    let display_name = method_name.to_string();

    let generic_scheme = build_generic_scheme(&combined_generic_param_infos);

    Ok((
        method_name,
        ImplMockMethodScheme {
            common: ImplCommonMethodScheme {
                vis: visibility,
                accessor_name,
                module_name,
                display_name,
                interface_name,
                generic_scheme,
                method_generic_params,
            },
            fake,
            spy,
        },
    ))
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_method_carries_both_halves() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl UserService {
                fn get_user(&self, id: u32) -> String {
                    todo!()
                }
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let scheme = ImplMockScheme::try_from(info).expect("conversion should succeed");

        let method = &scheme.methods[0];
        assert_eq!(method.0.to_string(), "get_user");
        assert_eq!(
            method.1.common.module_name.to_string(),
            "user_service__get_user_mock_module"
        );
        assert_eq!(method.1.common.accessor_name.to_string(), "get_user_mock");
        assert_eq!(
            method.1.common.interface_name.to_string(),
            "UserServiceGetUserMockInterface"
        );
        assert_eq!(
            method.1.fake.store_name.to_string(),
            "USER_SERVICE_GET_USER_FAKE_STORE"
        );
        assert_eq!(
            method.1.spy.store_name.to_string(),
            "USER_SERVICE_GET_USER_SPY_STORE"
        );
    }

    #[test]
    fn test_destructuring_param_is_rejected_by_the_spy_half() {
        let item_impl: syn::ItemImpl = parse_quote! {
            impl Foo {
                fn bar(&self, (a, b): (i32, i32)) {}
            }
        };
        let info = ImplBlockInfo::try_from(item_impl).expect("valid impl block");

        let Err(error) = ImplMockScheme::try_from(info) else {
            panic!("a destructuring parameter should be rejected");
        };
        assert!(error.to_string().contains("destructures its value"));
    }
}
