//! A spy needs a name for every parameter so it can forward it to
//! `internal_record_call`, so a wildcard pattern is rejected.

#[fnmock::spyable]
fn wildcard_param(_: String) {}

fn main() {}
