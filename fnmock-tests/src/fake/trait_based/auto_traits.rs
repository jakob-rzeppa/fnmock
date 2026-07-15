#[fnmock::fakeable]
fn auto_traits(value: Box<dyn Send>) -> usize {
    let _ = value;
    1
}

#[test]
fn test_auto_traits() {
    let result = auto_traits(Box::new(1usize));
    assert_eq!(result, 1);
}

#[test]
fn test_auto_traits_fake() {
    auto_traits_fake().setup(|value| {
        let _ = value;
        2
    });

    let result = auto_traits(Box::new(1usize));
    assert_eq!(result, 2);
}
