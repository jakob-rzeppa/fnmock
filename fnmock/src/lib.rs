pub mod generic_fake_store;
pub mod fake_store;

/// Re-export the derive macro so that users of the library can just use `fnmock::fakeable` instead of having to depend on `fnmock-derive` directly.
pub use fnmock_derive::*;
