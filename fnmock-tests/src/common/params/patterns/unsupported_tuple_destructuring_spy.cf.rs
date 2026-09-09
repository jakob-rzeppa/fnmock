//! A tuple-destructuring parameter is supported by `#[fnmock::fakeable]` (the pattern is
//! reproduced in the fake closure) but not by `#[fnmock::spyable]`: the matcher records one
//! named field per parameter, and a destructuring pattern leaves no name to match it under.
//! Take the tuple by a plain binding (`pair: (String, String)`) instead.

#[fnmock::spyable]
fn tuple_destructuring((left, right): (String, String)) -> String {
    format!("{left} {right}")
}

fn main() {}
