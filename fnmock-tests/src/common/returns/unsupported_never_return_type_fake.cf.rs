//! The never type `!` is not supported as a return type: there is no value
//! for a fake closure to produce when the real function never returns.

#[fnmock::fakeable]
fn returns_never(flag: bool) -> ! {
    panic!("flag: {flag}")
}

fn main() {}
