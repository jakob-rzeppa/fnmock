/// Regular function
mod fake_regular;
/// Async function
mod fake_regular_async;
/// Generic function using where clauses
mod fake_generic_where;

/// Function using dyn trait objects
mod fake_dyn;
// fnmock does not support `impl Trait` in parameters or return types. Use generic type parameters instead.

/// Generic function
mod fake_generic;
/// Generic async function
mod fake_generic_async;

/// Function with no parameters
mod fake_empty_params;

/// Function returning ()
mod fake_no_return_value;

/// Function with lifetime generics
mod fake_lifetimes;
