//! Shared handling of generic parameters, used for both free functions and impl block methods.
//!
//! The job of this module is to reduce a `syn::Generics` — which mixes lifetimes, type params,
//! const params, and a detached where clause — down to the flat list of type and const parameters
//! a fake is keyed by.

pub mod key_array;
mod merge;
pub mod params;
pub mod sanitized_params;
pub mod types;
