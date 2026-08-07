//! The generated module that holds a function's spy.
//!
//! One such module is generated per spied function. It contains the matcher its arguments are
//! tested against, the thread-local store holding the expectations, and the interface struct whose
//! `expect`/`expectf`/`expect_times`/`assert` methods wrap it. The whole module is
//! `#[cfg(test)]`-gated, so release builds compile no spy machinery at all.

pub mod generate;
mod helpers;
pub mod info;
