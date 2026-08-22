use crate::{
    item_info::{
        call_value::CallValue, function::FunctionInfo, generic_param_info::GenericParamInfo,
    },
    scheme::{
        common::{fn_closure_trait::check_type_is_supported, generic_scheme::build_generic_scheme},
        function::{
            common::FunctionCommonScheme,
            spy::{
                names::{
                    build_accessor_name, build_interface_name, build_matcher_name,
                    build_module_name, build_params_name, build_store_name,
                },
                param::{
                    build_reference_call_value, spy_param_type_for_params_tuple,
                    spy_param_type_with_lifetime_info,
                },
            },
        },
    },
};

mod names;
mod param;

pub struct FunctionSpyScheme {
    pub common: FunctionCommonScheme,

    pub store_name: syn::Ident,
    pub matcher_name: syn::Ident,
    /// The name of the wrapper struct `Params<'a>` is set to; see
    /// [`build_params_name`](names::build_params_name) for why it exists.
    pub params_name: syn::Ident,

    /// One identifier per parameter, in declaration order.
    pub param_idents: Vec<syn::Ident>,
    /// One type per parameter, in declaration order, with references stripped and lifetimes
    /// elided; see [`spy_param_type`](param::spy_param_type).
    pub param_types: Vec<syn::Type>,
    /// One type per parameter, in declaration order, for the element type of the matcher's
    /// `Params<'a>` tuple: like `param_types`, but with any lifetime substituted for the tuple's
    /// own `'a` instead of elided; see
    /// [`spy_param_type_for_params_tuple`](param::spy_param_type_for_params_tuple).
    pub params_tuple_types: Vec<syn::Type>,
    /// The expressions the injected call passes to `internal_record_call`, one per parameter, in
    /// declaration order; see [`build_reference_call_value`].
    pub reference_call_values: Vec<syn::Expr>,

    /// One expression per generic parameter, in declaration order, that renders it into the
    /// display name of an instantiation (e.g. `"i32"`, or `"5"` for a const generic's value). Only
    /// used when `common.generic_scheme` is `Some`.
    pub generic_display_fragments: Vec<syn::Expr>,

    /// Whether the matcher can offer `expect`'s `Predicate<..>`-based matching, alongside
    /// `expectf`.
    ///
    /// `false` when any parameter's type still names a lifetime after
    /// [`spy_param_type`](param::spy_param_type) strips and elides what it can: eliding a
    /// lifetime by omission only actually works inside a `Fn(..) -> ..` trait's own argument
    /// list, which is where `expectf`'s closure parameter lives but `expect`'s `Predicate<..>`
    /// bound and the matcher's own fields are not. Only `expectf` is offered in that case.
    pub supports_expect: bool,
}

impl TryFrom<FunctionInfo> for FunctionSpyScheme {
    type Error = syn::Error;

    fn try_from(value: FunctionInfo) -> Result<Self, Self::Error> {
        let module_name = build_module_name(&value.name);
        let store_name = build_store_name(&value.name);
        let accessor_name = build_accessor_name(&value.name);
        let interface_name = build_interface_name(&value.name);
        let matcher_name = build_matcher_name(&value.name);
        let params_name = build_params_name(&value.name);
        let display_name = value.name.to_string();

        let mut param_idents = Vec::with_capacity(value.params.len());
        let mut param_types = Vec::with_capacity(value.params.len());
        let mut params_tuple_types = Vec::with_capacity(value.params.len());
        let mut reference_call_values = Vec::with_capacity(value.params.len());
        let mut supports_expect = true;
        let params_tuple_lifetime: syn::Lifetime = syn::parse_quote!('a);

        for param in &value.params {
            let ident = match CallValue::try_from(&param.pat)? {
                CallValue::Ident(ident) => ident,
                CallValue::Tuple(_) | CallValue::Slice(_) => {
                    return Err(syn::Error::new_spanned(
                        &param.pat,
                        "The #[spyable] attribute only supports plain identifier parameters. This parameter destructures its value, so there is no name to match it under.",
                    ));
                }
            };

            let (param_type, needs_lifetime) = spy_param_type_with_lifetime_info(&param.ty);
            check_type_is_supported(&param_type)?;
            if needs_lifetime {
                supports_expect = false;
            }

            reference_call_values.push(build_reference_call_value(&ident, &param.ty));
            params_tuple_types.push(spy_param_type_for_params_tuple(
                &param.ty,
                &params_tuple_lifetime,
            ));
            param_idents.push(ident);
            param_types.push(param_type);
        }

        let generic_scheme = build_generic_scheme(&value.generic_params);
        let generic_display_fragments = value
            .generic_params
            .iter()
            .map(build_generic_display_fragment)
            .collect();

        Ok(FunctionSpyScheme {
            common: FunctionCommonScheme {
                vis: value.visibility,
                original: value.original,
                module_name,
                display_name,
                accessor_name,
                interface_name,
                generic_scheme,
            },
            store_name,
            matcher_name,
            params_name,
            param_idents,
            param_types,
            params_tuple_types,
            reference_call_values,
            generic_display_fragments,
            supports_expect,
        })
    }
}

/// Builds the expression that renders one generic parameter into an instantiation's display
/// name: the full type name for a type parameter (e.g. `"alloc::string::String"`), or the value
/// itself for a const parameter (e.g. `"5"`).
fn build_generic_display_fragment(info: &GenericParamInfo) -> syn::Expr {
    let ident = &info.ident;
    match &info.param {
        syn::GenericParam::Const(_) => syn::parse_quote! { #ident.to_string() },
        // Type params, and lifetimes (which `extract_generic_param_infos` never produces a
        // `GenericParamInfo` for in the first place).
        _ => syn::parse_quote! { ::std::any::type_name::<#ident>().to_string() },
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_standalone_non_generic_function() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn get_user(id: String, uuid: &str) -> String {
                todo!()
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionSpyScheme::try_from(info)
            .expect("conversion should succeed for a non-generic function");

        assert_eq!(scheme.common.module_name.to_string(), "get_user_spy_module");
        assert_eq!(scheme.common.display_name, "get_user");
        assert_eq!(scheme.common.accessor_name.to_string(), "get_user_spy");
        assert_eq!(
            scheme.common.interface_name.to_string(),
            "GetUserSpyInterface"
        );
        assert!(scheme.common.generic_scheme.is_none());
        assert_eq!(scheme.store_name.to_string(), "GET_USER_SPY_STORE");
        assert_eq!(scheme.matcher_name.to_string(), "GetUserMatcher");
        assert_eq!(
            scheme
                .param_idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["id".to_string(), "uuid".to_string()]
        );
        assert_eq!(
            scheme
                .param_types
                .iter()
                .map(|t| t.to_token_stream().to_string())
                .collect::<Vec<_>>(),
            vec!["String".to_string(), "str".to_string()]
        );
        assert_eq!(
            scheme
                .reference_call_values
                .iter()
                .map(|v| v.to_token_stream().to_string())
                .collect::<Vec<_>>(),
            vec!["& id".to_string(), "uuid".to_string()]
        );
    }

    #[test]
    fn test_generic_function_carries_generic_info_and_display_fragments_through() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo<T: 'static>(x: T) -> T {
                x
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionSpyScheme::try_from(info)
            .expect("conversion should succeed for a generic function");

        let generic_scheme = scheme
            .common
            .generic_scheme
            .as_ref()
            .expect("expected generic_scheme to be Some");
        assert_eq!(
            generic_scheme
                .idents
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>(),
            vec!["T".to_string()]
        );
        assert_eq!(scheme.generic_display_fragments.len(), 1);
        assert_eq!(
            scheme.generic_display_fragments[0]
                .to_token_stream()
                .to_string(),
            quote::quote!(::std::any::type_name::<T>().to_string()).to_string()
        );
    }

    #[test]
    fn test_const_generic_display_fragment_uses_the_value_not_the_type() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo<const C: usize>() -> usize {
                C
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionSpyScheme::try_from(info)
            .expect("conversion should succeed for a const-generic function");

        assert_eq!(
            scheme.generic_display_fragments[0]
                .to_token_stream()
                .to_string(),
            quote::quote!(C.to_string()).to_string()
        );
    }

    #[test]
    fn test_tuple_destructuring_param_is_rejected() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo((a, b): (i32, i32)) {}
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let result = FunctionSpyScheme::try_from(info);

        let Err(error) = result else {
            panic!(
                "a destructuring parameter should be rejected: a matcher needs one name per parameter"
            );
        };
        assert!(error.to_string().contains("#[spyable]"));
    }

    #[test]
    fn test_wildcard_param_is_rejected() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo(_: String) {}
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        assert!(FunctionSpyScheme::try_from(info).is_err());
    }

    #[test]
    fn test_impl_trait_param_is_rejected() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo(value: impl std::fmt::Display) {}
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        assert!(FunctionSpyScheme::try_from(info).is_err());
    }

    #[test]
    fn test_inferred_param_type_is_rejected() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo(value: _) {}
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        assert!(FunctionSpyScheme::try_from(info).is_err());
    }

    #[test]
    fn test_lifetime_bearing_param_type_disables_expect() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo(r: Ref<'_>) -> usize {
                todo!()
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionSpyScheme::try_from(info).expect("conversion should succeed");

        assert!(!scheme.supports_expect);
    }

    #[test]
    fn test_plain_type_supports_expect() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo(id: String) -> usize {
                todo!()
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionSpyScheme::try_from(info).expect("conversion should succeed");

        assert!(scheme.supports_expect);
    }

    #[test]
    fn test_params_tuple_type_substitutes_the_lifetime_instead_of_eliding_it() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn foo(r: Ref<'_>) -> usize {
                todo!()
            }
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme = FunctionSpyScheme::try_from(info).expect("conversion should succeed");

        // `Ref<>` (angle brackets kept, empty) not bare `Ref`: rustfmt normalizes an empty
        // `<>` away inside `quote!`/`parse_quote!` calls, so the expected value is written as
        // a plain string here rather than `quote::quote!(Ref<>).to_string()`.
        assert_eq!(
            scheme.param_types[0].to_token_stream().to_string(),
            "Ref < >"
        );
        assert_eq!(
            scheme.params_tuple_types[0].to_token_stream().to_string(),
            quote::quote!(Ref<'a>).to_string()
        );
    }

    #[test]
    fn test_zero_params() {
        let item_fn: syn::ItemFn = parse_quote! {
            fn ping() {}
        };
        let info = FunctionInfo::try_from(item_fn).expect("valid function");

        let scheme =
            FunctionSpyScheme::try_from(info).expect("conversion should succeed for zero params");

        assert!(scheme.param_idents.is_empty());
        assert!(scheme.param_types.is_empty());
        assert!(scheme.reference_call_values.is_empty());
    }
}
