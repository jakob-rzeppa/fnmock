//! Parsing of the annotated item into the information the generators need.
//!
//! Everything downstream of this module works off the `*Info` structs produced here rather than
//! off raw `syn` types, so signature validation happens in one place.

pub mod call_value;
pub mod function;
mod generics;
pub mod item_impl;
mod lifetimes;
pub mod params;
mod replace_self;
