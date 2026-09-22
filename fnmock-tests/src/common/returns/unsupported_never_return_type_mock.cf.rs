//! A mock is the intersection of what fake and spy support. Spy does not care
//! what a function returns, but fake has to produce a value, so there is
//! nothing for its closure to return when the function never does.
//! `#[fnmock::mockable]` rejects it with fake's message.

#[fnmock::mockable]
fn returns_never(flag: bool) -> ! {
    panic!("flag: {flag}")
}

fn main() {}
