struct GenericCombined<T> {
    value: T,
}

#[fnmock::fakeable]
impl<T: Clone + 'static> GenericCombined<T> {
    fn combine<U: 'static>(&self, other: U) -> (T, U) {
        (self.value.clone(), other)
    }
}

#[test]
fn test_generic_combined() {
    let s = GenericCombined {
        value: "Test".to_string(),
    };
    assert_eq!(s.combine(42), ("Test".to_string(), 42));
}

#[test]
fn test_generic_combined_fake() {
    GenericCombined::<String>::combine_fake::<i32>()
        .setup(|_, other| ("Fake".to_string(), other * 2));

    let s = GenericCombined {
        value: "Test".to_string(),
    };
    assert_eq!(s.combine(42), ("Fake".to_string(), 84));
}
