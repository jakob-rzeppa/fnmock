#[fnmock::fakeable]
fn const_generics_multiple<const A: usize, const B: usize>(value: String) -> String {
    format!("{} {} {}", value, A, B)
}

#[test]
fn test_const_generics_multiple() {
    let res = const_generics_multiple::<3, 5>("Test".to_string());
    assert_eq!(res, "Test 3 5");
}

#[test]
fn test_const_generics_multiple_fake() {
    const_generics_multiple_fake::<3, 5>().setup(|value| format!("Fake {} 3 5", value));

    let res = const_generics_multiple::<3, 5>("Test".to_string());
    assert_eq!(res, "Fake Test 3 5");
}

#[test]
fn test_const_generics_multiple_value_isolation() {
    const_generics_multiple_fake::<3, 5>().setup(|value| format!("Fake {}", value));

    // A fake was only set up for A=3, B=5, so a call with a different value for either
    // const parameter must still run the real implementation.
    let res = const_generics_multiple::<3, 7>("Test".to_string());
    assert_eq!(res, "Test 3 7");

    let res = const_generics_multiple::<4, 5>("Test".to_string());
    assert_eq!(res, "Test 4 5");

    // The fake for A=3, B=5 should remain unaffected.
    let res = const_generics_multiple::<3, 5>("Test".to_string());
    assert_eq!(res, "Fake Test");
}
