/// Validates that a function can be faked.
///
/// Checks that:
/// - The function is not async (faking async functions is not supported)
///
/// # Arguments
///
/// * `function` - The function to validate
///
/// # Returns
///
/// - `Ok(())` - If the function can be faked
/// - `Err(syn::Error)` - If the function cannot be faked, with an error message
pub(crate) fn validate_function_fakeable(function: &syn::ItemFn) -> syn::Result<()> {
    if function.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.asyncness,
            "Cannot fake async functions"
        ));
    }

    Ok(())
}
