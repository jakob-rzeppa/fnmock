//! The call recording injected at the top of the original function body.
//!
//! This is what makes callers of a spied function need no changes: the function keeps its
//! signature and its body, and the injected statement hands the call's arguments to the spy on the
//! way past. It is `#[cfg(test)]`-gated, so a release build runs the original body unchanged.

pub mod generate;
pub mod info;
