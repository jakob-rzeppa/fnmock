//! `Ref<'_>` is anonymous rather than named, but it still leaves a lifetime in
//! the parameter type, so it goes down the same substitution path.

struct Ref<'a>(&'a str);

#[fnmock::spyable]
fn elided_lifetime_param_type(r: Ref<'_>, tag: &str) -> usize {
    let _ = tag;
    r.0.len()
}

#[test]
fn test_elided_lifetime_param_type() {
    let spy = elided_lifetime_param_type_spy();
    spy.expectf(|r: &Ref<'_>, tag: &str| r.0 == "hi" && tag == "tag")
        .once();

    let owned = "hi".to_string();
    let res = elided_lifetime_param_type(Ref(&owned), "tag");

    assert_eq!(res, 2);
    spy.assert();
}
