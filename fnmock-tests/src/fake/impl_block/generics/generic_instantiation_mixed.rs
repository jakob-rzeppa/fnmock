use std::fmt::Display;

struct Foo<T, U> {
    value: T,
    value2: U,
}

#[fnmock::fakeable]
impl<U: Display + 'static> Foo<u8, U> {
    fn bar(&self) -> u8 {
        self.value
    }

    fn bar2(self) -> U {
        self.value2
    }
}

#[test]
fn test_foo_bar() {
    let foo = Foo::<u8, i32> {
        value: 1,
        value2: 2,
    };
    assert_eq!(foo.bar(), 1);
    assert_eq!(foo.bar2(), 2);
}

#[test]
fn test_foo_bar_fake() {
    Foo::<u8, i32>::bar_fake().setup(|_| 6);
    Foo::<u8, i32>::bar2_fake().setup(|_| 7);
    let foo = Foo::<u8, i32> {
        value: 1,
        value2: 2,
    };
    assert_eq!(foo.bar(), 6);
    assert_eq!(foo.bar2(), 7);
}
