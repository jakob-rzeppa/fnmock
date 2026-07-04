use quote::{ ToTokens, quote };
use syn::parse_quote;

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a `Generics` object
///
/// Returns a vector of `TypeParam` objects representing the generic type parameters
pub fn extract_generic_type_params(generics: &syn::Generics) -> syn::Result<Vec<syn::TypeParam>> {
    let mut type_params: Vec<syn::TypeParam> = generics.params
        .iter()
        .filter_map(|param| {
            match param {
                syn::GenericParam::Type(type_param) => Some(type_param.clone()),
                _ => None,
            }
        })
        .collect();

    merge_where_bounds_into_type_params(generics, &mut type_params);

    // Check if any of the type parameters have a lifetime bound. If so, we check if it is static. If not we panic, because we don't support non-static lifetimes in generic parameters for fakeable functions.
    for type_param in &type_params {
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

    Ok(type_params)
}

fn merge_where_bounds_into_type_params(
    generics: &syn::Generics,
    type_params: &mut Vec<syn::TypeParam>
) {
    let Some(where_clause) = &generics.where_clause else {
        return;
    };

    for predicate in &where_clause.predicates {
        let syn::WherePredicate::Type(type_predicate) = predicate else {
            continue;
        };

        let type_param = if
            let Some(existing) = type_params
                .iter_mut()
                .find(
                    |param|
                        param.ident.to_token_stream().to_string() ==
                        type_predicate.bounded_ty.to_token_stream().to_string()
                )
        {
            // If the type parameter already exists, use the existing one
            existing
        } else {
            panic!(
                "Type parameter {} in where clause does not exist in the generic parameters",
                type_predicate.bounded_ty.to_token_stream().to_string()
            );
        };

        for bound in &type_predicate.bounds {
            if
                type_param.bounds
                    .iter()
                    .any(|existing| {
                        existing.to_token_stream().to_string() ==
                            bound.to_token_stream().to_string()
                    })
            {
                continue;
            }

            type_param.bounds.push(bound.clone());
        }
    }
}

/// Extract the generic types (e.g. `T`, `U`) from a list of generic parameters (e.g. `T: Display + 'static`, `U: 'static`)
pub fn extract_generic_types_from_type_params(generic_params: &[syn::TypeParam]) -> Vec<syn::Type> {
    generic_params
        .iter()
        .map(|param| param.ident.clone())
        .map(|ident| parse_quote!(#ident))
        .collect()
}

/// Build TypeId array: [TypeId::of::<T>(), TypeId::of::<U>(), ...]
pub fn build_type_id_array(generic_idents: &[syn::Type]) -> Vec<syn::Expr> {
    generic_idents
        .iter()
        .map(|ident| {
            quote! { std::any::TypeId::of::<#ident>() }
        })
        .map(|ts| syn::parse2(ts))
        .collect::<syn::Result<_>>()
        .expect("Type ids must be parsable to an expression. This should not fail.")
}
