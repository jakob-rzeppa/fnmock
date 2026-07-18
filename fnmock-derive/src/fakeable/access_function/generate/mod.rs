//! Code generation for the accessor.
//!
//! A free function's accessor is a free function, while an impl block method's accessor is an
//! associated function on the same type, so the two cases are generated separately.

pub mod impl_block;
pub mod standalone;
