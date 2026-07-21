//! Reduction of a `syn::Generics` to the type and const parameters a fake is keyed by.

use crate::extract::generics::{
    merge::merge_where_bounds_into_type_params, sanitized_params::SanitizedGenericParams,
};

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a `Generics` object
///
/// Returns a vector of `TypeParam` objects representing the generic type parameters
///
/// Lifetime parameters are dropped: a fake is keyed by `TypeId`, which does not distinguish
/// lifetimes, so they cannot contribute to the key.
///
/// # Errors
///
/// Returns a spanned error if a type parameter carries a non-`'static` lifetime bound. Such a
/// bound cannot be honoured, because the store keys types by `TypeId`, which requires `'static`.
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

    // Check if any of the type parameters have a lifetime bound. If so, we check if it is static. If not we return an error, because we don't support non-static lifetimes in generic parameters for fakeable functions.
    for generic_param in &generic_params {
        if let syn::GenericParam::Type(type_param) = generic_param {
            for bound in &type_param.bounds {
                if let syn::TypeParamBound::Lifetime(lifetime) = bound {
                    if lifetime.ident != "static" {
                        return Err(
                            syn::Error::new_spanned(
                                lifetime,
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
