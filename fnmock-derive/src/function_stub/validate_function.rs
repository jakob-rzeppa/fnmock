/// Validates that a function can be stubbed.
///
/// Checks that:
/// - The function is not async (stubbing async functions is not supported)
///
/// # Arguments
///
/// * `function` - The function to validate
///
/// # Returns
///
/// - `Ok(())` - If the function can be stubbed
/// - `Err(syn::Error)` - If the function cannot be stubbed, with an error message
pub(crate) fn validate_function_stubbable(function: &syn::ItemFn) -> syn::Result<()> {
    if function.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &function.sig.asyncness,
            "Cannot stub async functions"
        ));
    }

    Ok(())
}
