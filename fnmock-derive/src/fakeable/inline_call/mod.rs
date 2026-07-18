//! The fake lookup injected at the top of the original function body.
//!
//! This is what makes callers of a faked function need no changes: the function keeps its
//! signature and its body, and the injected lookup diverts the call to the fake when one is
//! installed on the current thread. The lookup is `#[cfg(test)]`-gated, so a release build runs
//! the original body unchanged.

pub mod generate;
pub mod info;
