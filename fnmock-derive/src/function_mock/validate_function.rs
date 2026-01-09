use crate::param_utils::validate_static_params;

/// Validates that a function is suitable for mocking.
///
/// Performs the following checks:
/// - All non-ignored parameters are 'static (no references allowed)
///
/// # Arguments
///
/// * `input` - The function item to validate
/// * `ignore_indices` - Indices of parameters to skip validation for
///
/// # Returns
///
/// - `Ok(())` if the function is valid for mocking
/// - `Err(syn::Error)` with a descriptive error message if validation fails
pub(crate) fn validate_function_mockable(input: &syn::ItemFn, ignore_indices: &[usize]) -> syn::Result<()> {
    // Validate that all non-ignored parameters are 'static (no references)
    validate_static_params(&input.sig.inputs, ignore_indices)?;

    Ok(())
}
