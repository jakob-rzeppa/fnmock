#[fnmock::fakeable]
fn unused_lifetime<'a, 'b>(a: &'a str) -> &'a str {
    a
}

#[test]
fn test_unused_lifetime_no_fake() {
    let a = "seven";
    let result = unused_lifetime(a);
    assert_eq!(result, "seven");
}

#[test]
fn test_unused_lifetime_fake() {
    unused_lifetime_fake().setup(|_a| "Fake");
    let a = "seven";
    let result = unused_lifetime(a);
    assert_eq!(result, "Fake");
}
