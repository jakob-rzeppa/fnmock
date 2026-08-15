//! The accessor a test calls to reach a fake, e.g. `fetch_user_fake()`.
//!
//! The accessor is the only generated item a user names directly. It hands back the fake module's
//! interface struct, on which `setup`, `clear` and `is_set` live.

pub mod generate;
pub mod info;
