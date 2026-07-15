use crate::{
    extract::{ function::info::FunctionInfo, item_impl::info::ImplItemFnInfo },
    names::{
        NameType,
        build_access_function_name,
        build_impl_interface_struct_name,
        build_impl_module_name,
        build_interface_struct_name,
        build_module_name,
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
/// `impl<...>` block. In both cases `generic_types` is the full list of types (struct +
/// method, where applicable) needed to instantiate the fake module's interface struct.
#[derive(Clone)]
pub struct AccessFunctionGenericInfo {
    pub generic_types: Vec<syn::Type>,
    pub generic_params: Vec<syn::GenericParam>,
}

impl TryFrom<&FunctionInfo> for AccessFunctionInfo {
    type Error = syn::Error;

    fn try_from(function_info: &FunctionInfo) -> Result<Self, Self::Error> {
        let access_function_name = build_access_function_name(
            &function_info.name,
            NameType::Fake
        );
        let module_name = build_module_name(&function_info.name, NameType::Fake);
        let interface_struct_name = build_interface_struct_name(
            &function_info.name,
            NameType::Fake
        );

        Ok(AccessFunctionInfo {
            access_function_name,
            module_name,
            interface_struct_name,
            generic_info: function_info.generic_info.as_ref().map(|info| AccessFunctionGenericInfo {
                generic_types: info.types.clone(),
                generic_params: info.generic_params.clone(),
            }),
        })
    }
}

impl TryFrom<&ImplItemFnInfo> for AccessFunctionInfo {
    type Error = syn::Error;

    fn try_from(impl_item_fn_info: &ImplItemFnInfo) -> Result<Self, Self::Error> {
        let access_function_name = build_access_function_name(
            &impl_item_fn_info.method_name,
            NameType::Fake
        );
        let module_name = build_impl_module_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake
        );
        let interface_struct_name = build_impl_interface_struct_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake
        );

        Ok(AccessFunctionInfo {
            access_function_name,
            module_name,
            interface_struct_name,
            generic_info: impl_item_fn_info.generic_info.as_ref().map(|info| AccessFunctionGenericInfo {
                generic_types: info.types.clone(),
                generic_params: info.method_generic_params.clone(),
            }),
        })
    }
}
