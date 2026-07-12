#[fnmock::fakeable]
fn const_generics<const C: usize>(a: String) -> String {
    format!("{} {}", a, C)
}

#[test]
fn test_const_generics() {
    let res = const_generics::<5>("Test".to_string());
    assert_eq!(res, "Test 5");
}

#[test]
fn test_const_generics_fake() {
    const_generics_fake::<5>().setup(|a| {
        // You cant access C like this
        // format!("Fake {}", a, C)

        // So you have to hardcode it, but since you know the value of C and the fake is only for this specific value of C, it is not a problem
        format!("Fake {} {}", a, 5)
    });
    let res = const_generics::<5>("Test".to_string());
    assert_eq!(res, "Fake Test 5");
}
