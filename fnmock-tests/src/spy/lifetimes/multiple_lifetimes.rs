//! Two independent signature lifetimes get collapsed onto the single lifetime
//! of `Matcher::Params<'a>`. That is sound here because `Ref` is covariant in
//! its lifetime, so both arguments coerce down to the shorter borrow.

struct Ref<'a>(&'a str);

#[fnmock::spyable]
fn multiple_lifetimes<'a, 'b>(left: Ref<'a>, right: Ref<'b>) -> usize {
    left.0.len() + right.0.len()
}

#[test]
fn test_two_lifetimes_collapse_onto_the_matcher_lifetime() {
    let spy = multiple_lifetimes_spy();
    spy.expectf(|left: &Ref<'_>, right: &Ref<'_>| left.0 == "a" && right.0 == "bb")
        .once();

    let long = "a".to_string();
    let res = {
        let short = "bb".to_string();
        multiple_lifetimes(Ref(&long), Ref(&short))
    };

    assert_eq!(res, 3);
    spy.assert();
}
