use proc_macro::TokenStream;
use syn::{parse_macro_input};

mod param_utils;
mod use_tree_processor;
mod use_statement_processor;
mod inline_processor;
mod function_mock;
mod function_fake;
mod function_stub;
mod return_utils;

use crate::function_mock::{process_mock_function};
use crate::function_fake::{process_fake_function};
use crate::function_mock::mock_args::MockFunctionArgs;
use crate::function_stub::{process_stub_function};
use crate::inline_processor::process_inline;
use crate::use_statement_processor::process_use_statement;

/// Attribute macro that generates a mockable version of a function.
///
/// This macro modifies the original function to check (in test mode) if a mock implementation
/// has been configured and generates:
/// 1. The original function with injected mock checking logic (calls mock if set, otherwise executes normally)
/// 2. A `<function_name>_mock` module containing mock control methods
///
/// # Generated Mock Module Methods
///
/// - `setup(fn)` - Sets a custom implementation for the mock
/// - `clear()` - Resets the mock to its uninitialized state
/// - `is_set()` - Checks if the mock has been configured
/// - `assert_times(n)` - Verifies the function was called exactly n times
/// - `assert_with(params)` - Verifies the function was called with specific parameters
///
/// # Ignoring of parameters
///
/// If you don't want a parameter to be checked (for example if they have to be a reference or do not implement Clone / PartialEq),
/// you can ignore the parameter with:
///
/// ```ignore
/// #[mock_function(ignore = [db])]
/// pub(crate) fn fetch_user(db: SqlitePool /* Doesn't implement PartialEq */, id: u32) -> Result<String, String> {
///     // Real implementation
///     Ok(format!("user_{}", id))
/// }
/// ```
///
/// # Requirements
///
/// - Function must not have `self` parameters (standalone functions only)
/// - Not ignored function parameters must implement `Clone`, `Debug`, and `PartialEq` (for assertions)
/// - Not ignored function parameters must be `'static` (no references allowed - use owned types like `String` instead of `&str`)
///
/// # Example
///
/// ```ignore
/// use fnmock::derive::mock_function;
///
/// #[mock_function]
/// pub(crate) fn fetch_user(id: u32) -> Result<String, String> {
///     // Real implementation
///     Ok(format!("user_{}", id))
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     #[test]
///     fn test_with_mock() {
///         // Set up mock behavior
///         fetch_user_mock::setup(|id| {
///             Ok(format!("mock_user_{}", id))
///         });
///
///         // Call the original function (which will use the mock in tests)
///         let result = fetch_user(42);
///
///         // Verify behavior
///         assert_eq!(result, Ok("mock_user_42".to_string()));
///         fetch_user_mock::assert_times(1);
///         fetch_user_mock::assert_with(42);
///
///         // Clean up (not necessary since mocks are thread-local and reset between tests)
///         fetch_user_mock::clear();
///     }
/// }
/// ```
/// # Note
///
/// The mock module uses thread-local storage, so mocks are isolated
/// between tests but **not thread-safe** if the same function is mocked in parallel
/// test threads.
///
/// This means if you write a test that spawns multiple threads
/// and those threads all try to mock the same function simultaneously,
/// you could encounter undefined behavior.
/// The mock state is isolated between different test threads (good for test independence),
/// but not protected within a single test that uses multiple threads.
#[proc_macro_attribute]
pub fn mock_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let args = if attr.is_empty() {
        MockFunctionArgs { ignore: Vec::new() }
    } else {
        parse_macro_input!(attr as MockFunctionArgs)
    };

    match process_mock_function(input, args.ignore) {
        Ok(expanded) => TokenStream::from(expanded),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Attribute macro that generates a fakeable version of a function.
///
/// This macro modifies the original function to check (in test mode) if a fake implementation
/// has been configured and generates:
/// 1. The original function with injected fake checking logic (calls fake if set, otherwise executes normally)
/// 2. A `<function_name>_fake` module containing fake control methods
///
/// # Generated Fake Module Methods
///
/// - `setup(fn)` - Sets a custom implementation for the fake
/// - `clear()` - Resets the fake to its uninitialized state
/// - `is_set()` - Checks if the fake has been configured
/// - `get_implementation()` - Gets the current fake implementation
///
/// # Difference from Mocks
///
/// Fakes - in contrast to mocks - do not let you make assertions about if and how
/// the function was called. Fakes are simpler and only provide alternative implementations.
///
/// One important advantage of fakes is, that they **allow references as parameters**, unlike mocks.
/// This is the case, because they don't need to store the provided parameters and therefore don't cause lifetime issues.
///
/// # Requirements
///
/// - Function must not have `self` parameters (standalone functions only)
///
/// # Example
///
/// ```ignore
/// use fnmock::derive::fake_function;
///
/// #[fake_function]
/// pub(crate) fn add_two(x: i32) -> i32 {
///     x + 2
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     #[test]
///     fn test_with_fake() {
///         // Set up fake behavior
///         add_two_fake::setup(|x| x + 10);
///
///         // Call the original function (which will use the fake in tests)
///         let result = add_two(5);
///
///         // Verify behavior
///         assert_eq!(result, 15);
///
///         // Clean up (not necessary since fakes are thread-local and reset between tests)
///         add_two_fake::clear();
///     }
/// }
/// ```
/// # Note
///
/// The fake module uses thread-local storage, so fakes are isolated
/// between tests but **not thread-safe** if the same function is faked in parallel
/// test threads.
#[proc_macro_attribute]
pub fn fake_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    match process_fake_function(input) {
        Ok(expanded) => TokenStream::from(expanded),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Attribute macro that generates a stubbable version of a function.
///
/// This macro modifies the original function to check (in test mode) if a stub implementation
/// has been configured and generates:
/// 1. The original function with injected stub checking logic (calls stub if set, otherwise executes normally)
/// 2. A `<function_name>_stub` module containing stub control methods
///
/// # Generated Stub Module Methods
///
/// - `setup(return_value)` - Sets the predetermined return value for the stub
/// - `clear()` - Resets the stub to its uninitialized state
/// - `is_set()` - Checks if the stub has been configured
/// - `get_return_value()` - Gets the current stubbed return value
///
/// # Difference from Mocks and Fakes
///
/// Stubs - in contrast to mocks and fakes - provide canned responses without behavior verification or custom logic.
/// They simply return predetermined values to allow tests to proceed.
///
/// - **Mocks** track calls and allow assertions, and use custom implementations
/// - **Fakes** provide custom implementations without tracking
/// - **Stubs** only return predetermined values without custom logic or tracking
///
/// # Requirements
///
/// - Function must not have `self` parameters (standalone functions only)
/// - Return type must implement `Clone` (since the stub may be called multiple times)
///
/// # Example
///
/// ```ignore
/// use fnmock::derive::stub_function;
///
/// #[stub_function]
/// pub(crate) fn get_config() -> String {
///     // Real implementation that reads from file
///     std::fs::read_to_string("config.json").unwrap()
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     #[test]
///     fn test_with_stub() {
///         // Set up stub return value
///         get_config_stub::setup("test_config".to_string());
///
///         // Call the original function (which will use the stub in tests)
///         let result = get_config();
///
///         // Verify result
///         assert_eq!(result, "test_config");
///
///         // Clean up (not necessary since stubs are thread-local and reset between tests)
///         get_config_stub::clear();
///     }
/// }
/// ```
/// # Note
///
/// The stub module uses thread-local storage, so stubs are isolated
/// between tests but **not thread-safe** if the same function is stubbed in parallel
/// test threads.
#[proc_macro_attribute]
pub fn stub_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    match process_stub_function(input) {
        Ok(expanded) => TokenStream::from(expanded),
        Err(e) => e.to_compile_error().into(),
    }
}
