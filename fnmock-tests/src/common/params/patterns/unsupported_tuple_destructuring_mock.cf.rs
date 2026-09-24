//! A mock is the intersection of what fake and spy support. Fake reproduces a
//! tuple-destructuring pattern in its closure just fine, but spy has no name to
//! match the parameter under, so `#[fnmock::mockable]` rejects it with spy's
//! message. Take the tuple by a plain binding (`pair: (String, String)`) instead.

#[fnmock::mockable]
fn tuple_destructuring((left, right): (String, String)) -> String {
    format!("{left} {right}")
}

fn main() {}
