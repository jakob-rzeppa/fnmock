use crate::extract::generics::{
    merge::merge_where_bounds_into_type_params, sanitized_params::SanitizedGenericParams,
};

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a `Generics` object
///
/// Returns a vector of `TypeParam` objects representing the generic type parameters
pub fn extract_generic_type_and_const_params(
    generics: &syn::Generics,
) -> syn::Result<SanitizedGenericParams> {
    let mut generic_params: Vec<syn::GenericParam> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Type(_) => Some(param.clone()),
            syn::GenericParam::Const(_) => Some(param.clone()),
            _ => None,
        })
        .collect();

    merge_where_bounds_into_type_params(generics, &mut generic_params);

    // Check if any of the type parameters have a lifetime bound. If so, we check if it is static. If not we return a error, because we don't support non-static lifetimes in generic parameters for fakeable functions.
    for generic_param in &generic_params {
        if let syn::GenericParam::Type(type_param) = generic_param {
            for bound in &type_param.bounds {
                if let syn::TypeParamBound::Lifetime(lifetime) = bound {
                    if lifetime.ident != "static" {
                        return Err(
                            syn::Error::new_spanned(
                                &lifetime,
                                format!(
                                    "Non-static lifetime '{}' found in generic parameter '{}'. Only 'static lifetimes are supported in generic parameters for fakeable functions.",
                                    lifetime.ident,
                                    type_param.ident
                                )
                            )
                        );
                    }
                }
            }
        }
    }

    SanitizedGenericParams::new(generic_params)
}
