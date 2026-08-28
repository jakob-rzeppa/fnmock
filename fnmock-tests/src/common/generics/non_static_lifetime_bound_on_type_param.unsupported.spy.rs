//! A non-`'static` lifetime bound on a type parameter is rejected up front
//! rather than failing inside the generated module, matching what `#[fakeable]`
//! already does.

#[fnmock::spyable]
fn non_static_lifetime_bound_on_type_param<'a, T: 'a>(a: T) {
    let _ = a;
}

fn main() {}
