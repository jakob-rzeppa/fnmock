//! A generic parameter list narrowed to what a fake can be keyed by.

/// A struct that holds the sanitized generic parameters of a function.
///
/// This means that the generic parameters have been filtered to only include type and const parameters,
/// and any where bounds have been merged into the type parameters.
///
/// Downstream code takes this type rather than a bare `Vec<syn::GenericParam>` so that "lifetimes
/// have already been stripped" is a property of the type instead of a convention every caller has
/// to remember.
pub struct SanitizedGenericParams {
    generic_params: Vec<syn::GenericParam>,
}

impl SanitizedGenericParams {
    /// Create a new `SanitizedGenericParams` from a vector of `GenericParam`.
    ///
    /// Returns an error if any of the generic parameters are lifetime parameters. Callers are
    /// expected to strip lifetime parameters beforehand, so hitting this is an internal fnmock bug
    /// rather than a user error.
    pub fn new(generic_params: Vec<syn::GenericParam>) -> syn::Result<Self> {
        for param in &generic_params {
            if !matches!(
                param,
                syn::GenericParam::Type(_) | syn::GenericParam::Const(_)
            ) {
                return Err(syn::Error::new_spanned(
                    param,
                    "internal error: SanitizedGenericParams may only contain type and const parameters, but a lifetime parameter was found. This is a bug in fnmock; please report it.",
                ));
            }
        }

        Ok(Self { generic_params })
    }

    /// Borrow the parameters, in declaration order.
    pub fn get_generic_params(&self) -> &Vec<syn::GenericParam> {
        &self.generic_params
    }

    /// Chain the generic parameters of another `SanitizedGenericParams` into this one.
    /// This is used to combine the generic parameters of a struct and a method.
    /// The method generics will be appended to the struct generics, in the order of struct generics followed by method generics.
    pub fn combine(&self, other: &SanitizedGenericParams) -> Self {
        Self {
            generic_params: self
                .generic_params
                .iter()
                .chain(other.generic_params.iter())
                .cloned()
                .collect(),
        }
    }

    /// Whether there is nothing left to key a fake by, meaning a plain `FakeStore` will do.
    pub fn is_empty(&self) -> bool {
        self.generic_params.is_empty()
    }

    /// The number of parameters, which becomes the `GENERIC_COUNT` of the generated store.
    pub fn len(&self) -> usize {
        self.generic_params.len()
    }

    /// Consume the wrapper and hand back the parameters, for embedding in generated code.
    pub fn into_generic_params(self) -> Vec<syn::GenericParam> {
        self.generic_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rejects_lifetime_parameter() {
        let lifetime: syn::GenericParam = syn::parse_quote!('a);

        let result = SanitizedGenericParams::new(vec![lifetime]);

        assert!(
            result.is_err(),
            "expected SanitizedGenericParams::new to error (not panic) on a lifetime parameter"
        );
    }

    #[test]
    fn test_new_accepts_type_and_const_parameters() {
        let type_param: syn::GenericParam = syn::parse_quote!(T);
        let const_param: syn::GenericParam = syn::parse_quote!(const N: usize);

        let result = SanitizedGenericParams::new(vec![type_param, const_param]);

        assert!(
            result.is_ok(),
            "expected type and const parameters to be accepted"
        );
    }
}
