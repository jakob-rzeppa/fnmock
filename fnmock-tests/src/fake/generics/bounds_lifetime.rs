#[fnmock::fakeable]
fn bounds_lifetime<'a: 'b, 'b>(a: &'a str, _b: &'b str) -> &'b str {
    a
}

#[test]
fn test_bounds_lifetime_no_fake() {
    let a = "A";
    let b = "B";
    let result = bounds_lifetime(a, b);
    assert_eq!(result, "A");
}

#[test]
fn test_bounds_lifetime_fake() {
    // The lifetime boumds cannot be expressed in the fake closure signature. This is a limitation of the current implementation,
    // but it is not a problem for the generated code because the fake closure is only ever called with concrete lifetimes.
    bounds_lifetime_fake().setup(|_a, _b| "Fake");
    let a = "A";
    let b = "B";
    let result = bounds_lifetime(a, b);
    assert_eq!(result, "Fake");
}

#[fnmock::fakeable]
fn bounds_lifetime_where<'a, 'b>(a: &'a str, _b: &'b str) -> &'b str
where
    'a: 'b,
{
    a
}

#[test]
fn test_bounds_lifetime_where_no_fake() {
    let a = "A";
    let b = "B";
    let result = bounds_lifetime_where(a, b);
    assert_eq!(result, "A");
}

#[test]
fn test_bounds_lifetime_where_fake() {
    bounds_lifetime_where_fake().setup(|_a, _b| "Fake");
    let a = "A";
    let b = "B";
    let result = bounds_lifetime_where(a, b);
    assert_eq!(result, "Fake");
}
