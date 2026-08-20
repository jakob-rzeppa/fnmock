struct Foo<T> {
    value: T,
}

#[fnmock::fakeable]
impl Foo<u8> {
    fn bar(&self) -> u8 {
        self.value
    }
}

#[test]
fn test_foo_bar() {
    let f8 = Foo::<u8> { value: 1 };
    assert_eq!(f8.bar(), 1);
}

#[test]
fn test_foo_bar_fake() {
    Foo::<u8>::bar_fake().setup(|_| 9);
    assert_eq!(Foo::<u8> { value: 1 }.bar(), 9);
}
