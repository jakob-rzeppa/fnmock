use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Expr, Ident};

/// Processes a function path expression and generates the conditional fake selection code.
///
/// Takes a function path and creates a block that conditionally evaluates to either
/// the original function or the fake version based on the test configuration.
///
/// # Arguments
///
/// * `input` - The expression passed to the macro (should be a path expression)
///
/// # Returns
///
/// - `Ok(TokenStream2)` - The generated conditional code block
/// - `Err(syn::Error)` - If the input is not a valid function path
///
/// # Generated Code
///
/// ```ignore
/// {
///     #[cfg(not(test))]
///     { original::path::function }
///     #[cfg(test)]
///     { original::path::function_fake }
/// }
/// ```
pub(crate) fn process_use_fake_inline(input: &Expr) -> syn::Result<TokenStream2> {
    // Extract the function path
    let fn_path = match input {
        Expr::Path(path) => path,
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "use_fake_inline expects a function identifier or path"
            ));
        }
    };

    // Get the function name (last segment of the path)
    let fn_name = match fn_path.path.segments.last() {
        Some(segment) => &segment.ident,
        None => {
            return Err(syn::Error::new_spanned(
                &fn_path.path,
                "Could not extract function name from path"
            ));
        }
    };

    // Create the fake function name
    let fake_fn_name = Ident::new(&format!("{}_fake", fn_name), fn_name.span());

    // Clone the path for the fake version and replace the last segment
    let mut fake_path = fn_path.clone();
    if let Some(last_segment) = fake_path.path.segments.last_mut() {
        last_segment.ident = fake_fn_name;
    }

    Ok(quote! {
        {
            #[cfg(not(test))]
            { #fn_path }
            #[cfg(test)]
            { #fake_path }
        }
    })
}
