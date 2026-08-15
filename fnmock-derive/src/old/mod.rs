//! The pre-restructure implementation of `#[fakeable]`/`#[spyable]`, kept as a reference to port
//! logic from and diff generated output against while the new expansion pipeline is built. See
//! `docs/superpowers/specs` (or the session's plan file) for the restructure this supports.
//!
//! Deleted once the new pipeline (`crate::expand`) is verified equivalent.

pub mod extract;
pub mod fakeable;
mod module_builder;
mod names;
pub mod spyable;
