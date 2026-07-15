use quote::{ ToTokens, quote };
use syn::parse_quote;

/// A struct that holds the sanitized generic parameters of a function.
///
/// This means that the generic parameters have been filtered to only include type and const parameters,
/// and any where bounds have been merged into the type parameters.
pub struct SanitizedGenericParams {
    generic_params: Vec<syn::GenericParam>,
}

impl SanitizedGenericParams {
    /// Create a new `SanitizedGenericParams` from a vector of `GenericParam`.
    /// This function will panic if any of the generic parameters are lifetime parameters.
    pub fn new(generic_params: Vec<syn::GenericParam>) -> Self {
        assert!(
            generic_params
                .iter()
                .all(|param|
                    matches!(param, syn::GenericParam::Type(_) | syn::GenericParam::Const(_))
                ),
            "SanitizedGenericParams should only contain type and const parameters. Lifetime parameters are not allowed."
        );

        Self { generic_params }
    }

    pub fn get_generic_params(&self) -> &Vec<syn::GenericParam> {
        &self.generic_params
    }

    /// Chain the generic parameters of another `SanitizedGenericParams` into this one.
    /// This is used to combine the generic parameters of a struct and a method.
    /// The method generics will be appended to the struct generics, in the order of struct generics followed by method generics.
    pub fn combine(&self, other: &SanitizedGenericParams) -> Self {
        Self {
            generic_params: self.generic_params
                .iter()
                .chain(other.generic_params.iter())
                .cloned()
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.generic_params.is_empty()
    }

    pub fn len(&self) -> usize {
        self.generic_params.len()
    }

    pub fn to_generic_params(self) -> Vec<syn::GenericParam> {
        self.generic_params
    }
}

/// Extract the generic type parameters (e.g. `T: Display + 'static`, `U: 'static`) from a `Generics` object
///
/// Returns a vector of `TypeParam` objects representing the generic type parameters
pub fn extract_generic_type_and_const_params(
    generics: &syn::Generics
) -> syn::Result<SanitizedGenericParams> {
    let mut generic_params: Vec<syn::GenericParam> = generics.params
        .iter()
        .filter_map(|param| {
            match param {
                syn::GenericParam::Type(_) => Some(param.clone()),
                syn::GenericParam::Const(_) => Some(param.clone()),
                _ => None,
            }
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

    Ok(SanitizedGenericParams::new(generic_params))
}

/// Merge the where bounds into the type parameters.
/// This is necessary because the where bounds are not included in the generic parameters, but we need them to generate the fakeable function.
fn merge_where_bounds_into_type_params(
    generics: &syn::Generics,
    type_params: &mut Vec<syn::GenericParam>
) {
    let Some(where_clause) = &generics.where_clause else {
        return;
    };

    for predicate in &where_clause.predicates {
        let syn::WherePredicate::Type(type_predicate) = predicate else {
            continue;
        };

        let type_param = if
            let Some(syn::GenericParam::Type(existing)) = type_params.iter_mut().find(|param| {
                match param {
                    syn::GenericParam::Type(type_param) => {
                        type_param.ident.to_token_stream().to_string() ==
                            type_predicate.bounded_ty.to_token_stream().to_string()
                    }
                    _ => false,
                }
            })
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
///
/// For `const C: usize` this will extract `C`
pub fn extract_generic_types_from_generic_params(
    generic_params: &SanitizedGenericParams
) -> Vec<syn::Type> {
    generic_params
        .get_generic_params()
        .iter()
        .map(|param| {
            match param {
                syn::GenericParam::Type(type_param) => {
                    let type_ident = &type_param.ident;
                    parse_quote!(#type_ident)
                }
                syn::GenericParam::Const(const_param) => {
                    let const_ident = &const_param.ident;
                    parse_quote!(#const_ident)
                }
                _ =>
                    unreachable!(
                        "SanitizedGenericParams should only contain type and const parameters. Lifetime parameters are not allowed."
                    ),
            }
        })
        .collect()
}

/// Build a `GenericKeyPart` array: [GenericKeyPart::Type(TypeId::of::<T>()), GenericKeyPart::Const(ConstValue::new(C)), ...]
///
/// Type parameters are keyed by their `TypeId`. Const parameters are keyed by their actual value (via
/// `fnmock::generic_fake_store::ConstValue::new`), not just the `TypeId` of their type — otherwise every
/// value of e.g. `const C: usize` would collapse onto the single key `TypeId::of::<usize>()`.
pub fn build_generic_key_array(generic_idents: &SanitizedGenericParams) -> Vec<syn::Expr> {
    generic_idents
        .get_generic_params()
        .iter()
        .map(|param| {
            match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = &type_param.ident;
                    quote! { fnmock::generic_fake_store::key::GenericKeyPart::Type(std::any::TypeId::of::<#ident>()) }
                }
                syn::GenericParam::Const(const_param) => {
                    let const_ident = &const_param.ident;
                    quote! {
                        fnmock::generic_fake_store::key::GenericKeyPart::Const(
                            fnmock::generic_fake_store::key::ConstValue::new(#const_ident)
                        )
                    }
                }
                _ =>
                    unreachable!(
                        "SanitizedGenericParams should only contain type and const parameters. Lifetime parameters are not allowed."
                    ),
            }
        })
        .map(|ts| syn::parse2(ts))
        .collect::<syn::Result<_>>()
        .expect("Generic key parts must be parsable to an expression. This should not fail.")
}
