//! The generated module that holds a function's fake.
//!
//! One such module is generated per faked function. It contains the thread-local store and the
//! interface struct whose `setup`/`clear`/`is_set`/`get` methods wrap it. The whole module is
//! `#[cfg(test)]`-gated, so release builds compile no fake machinery at all.

pub mod generate;
pub mod info;
