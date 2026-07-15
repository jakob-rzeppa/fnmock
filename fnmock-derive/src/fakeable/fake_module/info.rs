use crate::{
    extract::{ function::info::FunctionInfo, item_impl::info::ImplItemFnInfo },
    names::{
        NameType,
        build_impl_interface_struct_name,
        build_impl_module_name,
        build_impl_store_name,
        build_interface_struct_name,
        build_module_name,
        build_store_name,
    },
};

/// Information needed to generate a fake module (the `thread_local` store + interface struct).
#[derive(Clone)]
pub struct FakeModuleInfo {
    pub module_name: syn::Ident,
    pub store_name: syn::Ident,
    pub display_name: String,
    pub interface_struct_name: syn::Ident,
    pub fn_closure_trait: syn::TraitBound,
    pub generic_info: Option<FakeModuleGenericInfo>,
}

/// Information about the generic parameters for a fake module.
///
/// If it is a struct method fake, the generic parameters from the struct come first, followed by the generic parameters from the method.
#[derive(Clone)]
pub struct FakeModuleGenericInfo {
    pub generic_count: usize,

    /// The types of the generic parameters.
    pub generic_types: Vec<syn::Type>,

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
        let interface_struct_name = build_interface_struct_name(
            &function_info.name,
            NameType::Fake
        );

        Ok(FakeModuleInfo {
            module_name,
            store_name,
            display_name,
            interface_struct_name,
            fn_closure_trait: function_info.fn_closure_trait.clone(),
            generic_info: function_info.generic_info.as_ref().map(|info| FakeModuleGenericInfo {
                generic_count: info.count,
                generic_types: info.types.clone(),
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
            NameType::Fake
        );
        let store_name = build_impl_store_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake
        );
        let display_name = format!(
            "{} {}",
            impl_item_fn_info.struct_name,
            impl_item_fn_info.method_name
        );
        let interface_struct_name = build_impl_interface_struct_name(
            &impl_item_fn_info.struct_name,
            &impl_item_fn_info.method_name,
            NameType::Fake
        );

        Ok(FakeModuleInfo {
            module_name,
            store_name,
            display_name,
            interface_struct_name,
            fn_closure_trait: impl_item_fn_info.fn_closure_trait.clone(),
            generic_info: impl_item_fn_info.generic_info.as_ref().map(|info| FakeModuleGenericInfo {
                generic_count: info.count,
                generic_types: info.types.clone(),
                generic_params: info.generic_params.clone(),
                generic_keys: info.generic_keys.clone(),
            }),
        })
    }
}
