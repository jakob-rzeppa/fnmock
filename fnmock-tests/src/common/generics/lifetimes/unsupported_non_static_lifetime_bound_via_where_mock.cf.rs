//! `T: 'a` spelled in the `where` clause is rejected exactly like the inline spelling: the
//! `where` clause has no `'a: 'static` predicate, so `'a` stays a non-`'static` lifetime.

#[fnmock::mockable]
fn non_static_lifetime_bound_via_where<'a, T>(value: &'a T) -> String
where
    T: 'a + std::fmt::Display,
{
    format!("{value}")
}

fn main() {}
