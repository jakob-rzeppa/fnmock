//! The accessor a test reaches a function's spy through.
//!
//! One free function per spied function, named after it (`get_user` gets `get_user_spy()`), which
//! hands back the interface struct expectations are set on. It is `#[cfg(test)] pub(crate)`, so it
//! can be called from any module in the crate but exists in no other build.

pub mod generate;
pub mod info;
