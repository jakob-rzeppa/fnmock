//! Parsing of the annotated item into the information the generators need.
//!
//! Everything downstream of this module works off the `*Info` structs produced here rather than
//! off raw `syn` types, so signature validation happens in one place.

mod fn_closure_trait;
pub mod function;
mod generics;
pub mod item_impl;
mod lifetimes;
mod params;
mod replace_self;
