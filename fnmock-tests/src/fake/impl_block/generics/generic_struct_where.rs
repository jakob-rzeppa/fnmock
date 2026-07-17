struct GenericStructWhere<T> {
    value: T,
}

#[fnmock::fakeable]
impl<T> GenericStructWhere<T>
where
    T: Clone + 'static,
{
    fn get(&self) -> T {
        self.value.clone()
    }
}

#[test]
fn test_generic_struct_where() {
    let s = GenericStructWhere {
        value: "Test".to_string(),
    };
    assert_eq!(s.get(), "Test");
}

#[test]
fn test_generic_struct_where_fake() {
    GenericStructWhere::<String>::get_fake().setup(|_| "Fake".to_string());

    let s = GenericStructWhere {
        value: "Test".to_string(),
    };
    assert_eq!(s.get(), "Fake");
}
