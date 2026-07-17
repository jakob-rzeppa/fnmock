struct GenericCombinedWhere<T> {
    value: T,
}

#[fnmock::fakeable]
impl<T> GenericCombinedWhere<T>
where
    T: Clone + 'static,
{
    fn combine<U>(&self, other: U) -> (T, U)
    where
        U: 'static,
    {
        (self.value.clone(), other)
    }
}

#[test]
fn test_generic_combined_where() {
    let s = GenericCombinedWhere {
        value: "Test".to_string(),
    };
    assert_eq!(s.combine(42), ("Test".to_string(), 42));
}

#[test]
fn test_generic_combined_where_fake() {
    GenericCombinedWhere::<String>::combine_fake::<i32>()
        .setup(|_, other| ("Fake".to_string(), other * 2));

    let s = GenericCombinedWhere {
        value: "Test".to_string(),
    };
    assert_eq!(s.combine(42), ("Fake".to_string(), 84));
}
