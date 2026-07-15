struct GenericMethodWhere;

#[fnmock::fakeable]
impl GenericMethodWhere {
    fn echo<T>(&self, a: T) -> T where T: 'static {
        a
    }
}

#[test]
fn test_generic_method_where() {
    let s = GenericMethodWhere;
    assert_eq!(s.echo("Test".to_string()), "Test");
}

#[test]
fn test_generic_method_where_fake() {
    GenericMethodWhere::echo_fake::<String>().setup(|_, a| format!("Fake {}", a));

    let s = GenericMethodWhere;
    assert_eq!(s.echo("Test".to_string()), "Fake Test");
}
