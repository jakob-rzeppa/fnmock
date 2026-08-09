//! The lifetime is nested inside a container rather than being the parameter's
//! own. `strip_reference` removes only the outer `&`, so `'a` survives in
//! `Vec<&'a str>` and the substitution still has to reach it.

#[fnmock::spyable]
fn nested_lifetime_in_container<'a>(items: &'a [&'a str]) -> usize {
    items.len()
}

#[test]
fn test_nested_lifetime_in_container() {
    let spy = nested_lifetime_in_container_spy();
    spy.expectf(|items: &[&str]| items == ["a", "b"]).once();

    let owned = ["a".to_string(), "b".to_string()];
    let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
    let res = nested_lifetime_in_container(&borrowed);

    assert_eq!(res, 2);
    spy.assert();
}
