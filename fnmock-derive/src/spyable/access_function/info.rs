//! The information needed to generate a spy's accessor.

use crate::{
    extract::function::info::FunctionInfo,
    names::{NameType, build_access_function_name, build_interface_struct_name, build_module_name},
};

/// Information needed to generate an access function for a spy (e.g. `get_user_spy()`).
#[derive(Clone)]
pub struct SpyAccessFunctionInfo {
    /// Access function name for the spy interface (e.g. "get_user_spy").
    pub access_function_name: syn::Ident,

    /// The name of the spy module this access function reaches into.
    pub module_name: syn::Ident,

    /// The name of the struct that provides the API for setting up and accessing the spy implementation.
    pub interface_struct_name: syn::Ident,
}

impl TryFrom<&FunctionInfo> for SpyAccessFunctionInfo {
    type Error = syn::Error;

    fn try_from(function_info: &FunctionInfo) -> Result<Self, Self::Error> {
        Ok(SpyAccessFunctionInfo {
            access_function_name: build_access_function_name(&function_info.name, NameType::Spy),
            module_name: build_module_name(&function_info.name, NameType::Spy),
            interface_struct_name: build_interface_struct_name(&function_info.name, NameType::Spy),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::function::extract_function_info;

    #[test]
    fn test_try_from_function_info_standalone_function() {
        let item_fn: syn::ItemFn = syn::parse_quote! {
            fn get_user(id: String) -> String {
                todo!()
            }
        };
        let function_info =
            extract_function_info(&item_fn, NameType::Spy).expect("valid standalone function");

        let info = SpyAccessFunctionInfo::try_from(&function_info)
            .expect("conversion should succeed for a standalone function");

        assert_eq!(info.access_function_name.to_string(), "get_user_spy");
        assert_eq!(info.module_name.to_string(), "get_user_spy_module");
        assert_eq!(
            info.interface_struct_name.to_string(),
            "GetUserSpyInterface"
        );
    }
}
