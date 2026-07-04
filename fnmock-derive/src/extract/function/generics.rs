use crate::extract::{
    function::info::FunctionGenericInfo,
    generic::{
        build_type_id_array,
        extract_generic_type_params,
        extract_generic_types_from_type_params,
    },
};

/// Extracts the generic information from a `Generics` object, including the count of generic parameters, the generic type parameters themselves, their identifiers, and their corresponding `TypeId` expressions.
///
/// This is used for free functions and not impl blocks, as impl blocks require special handling to combine the generic parameters from both the struct and the method.
pub fn extract_generic_function_info(
    generics: &syn::Generics
) -> syn::Result<Option<FunctionGenericInfo>> {
    let type_params = extract_generic_type_params(generics)?;

    if type_params.is_empty() {
        return Ok(None);
    }

    let types = extract_generic_types_from_type_params(&type_params);
    let type_ids = build_type_id_array(&types);

    Ok(
        Some(FunctionGenericInfo {
            count: type_params.len(),
            type_params,
            types,
            type_ids,
        })
    )
}
