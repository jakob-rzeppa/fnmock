/// No generics, no async
mod fake_regular_regular;
/// Struct generic, method regular, no async
mod fake_generic_regular;
/// Struct regular, method generic, no async
mod fake_regular_generic;
/// Struct generic, method generic, no async
mod fake_generic_generic;

/// Struct regular, method regular, async
mod fake_regular_regular_async;
/// Struct generic, method regular, async
mod fake_generic_regular_async;
/// Struct regular, method generic, async
mod fake_regular_generic_async;
/// Struct generic, method generic, async
mod fake_generic_generic_async;

/// A consuming `self` receiver
mod fake_consuming_self;
/// A mutable `&mut self` receiver
mod fake_mut_self;
/// No `self` receiver -> associated function
mod fake_associated_function;
/// Using Self type in method parameters
mod fake_self_as_param_type;

/// Method using dyn trait objects
mod fake_dyn;
// fnmock does not support `impl Trait` in parameters or return types. Use generic type parameters instead.

/// Returning ()
mod fake_returning_nothing;
/// Returning Self
mod fake_returning_self;
/// Returning generic Self
mod fake_returning_self_generic;
/// Returning Self nested in other types like Result<Self, E> or Option<Self>
mod fake_returning_self_nested;

/// Different fakes for different generic type parameters
mod fake_different_fakes_for_generics;

/// Using where clauses for generic type parameters
mod fake_generic_where;
