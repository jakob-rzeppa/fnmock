//! Type parameters on a spied function must be `'static`. The store is keyed by
//! `TypeId`, and `Expectation<M>` requires `M: Any`, so a matcher generic over a
//! non-`'static` `T` cannot exist.

#[fnmock::spyable]
fn non_static_type_param<T>(a: T) {
    let _ = a;
}

fn main() {}
