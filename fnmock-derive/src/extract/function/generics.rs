use crate::extract::{
    function::info::FunctionGenericInfo,
    generic::{
        build_type_id_array,
        extract_generic_type_and_const_params,
        extract_generic_types_from_generic_params,
    },
};

/// Extracts the generic information from a `Generics` object, including the count of generic parameters, the generic type parameters themselves, their identifiers, and their corresponding `TypeId` expressions.
///
/// This is used for free functions and not impl blocks, as impl blocks require special handling to combine the generic parameters from both the struct and the method.
pub fn extract_generic_function_info(
    generics: &syn::Generics
) -> syn::Result<Option<FunctionGenericInfo>> {
    let generic_params = extract_generic_type_and_const_params(generics)?;

    if generic_params.is_empty() {
        return Ok(None);
    }

    let types = extract_generic_types_from_generic_params(&generic_params);
    let type_ids = build_type_id_array(&generic_params);

    Ok(
        Some(FunctionGenericInfo {
            count: generic_params.len(),
            generic_params: generic_params.to_generic_params(),
            types,
            type_ids,
        })
    )
}
