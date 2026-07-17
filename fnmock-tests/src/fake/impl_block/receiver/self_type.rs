#[derive(Debug, PartialEq)]
struct SelfType {
    value: i32,
}

#[fnmock::fakeable]
impl SelfType {
    fn create(value: i32) -> Self {
        Self { value }
    }

    fn combine(&self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

#[test]
fn test_self_type() {
    let a = SelfType::create(42);
    let b = SelfType::create(8);
    assert_eq!(a.combine(b), SelfType { value: 50 });
}

#[test]
fn test_self_type_fake() {
    SelfType::create_fake().setup(|value| SelfType { value: value * 2 });
    SelfType::combine_fake().setup(|_, _| SelfType { value: 100 });

    let a = SelfType::create(42);
    assert_eq!(a, SelfType { value: 84 });

    let b = SelfType::create(8);
    assert_eq!(a.combine(b), SelfType { value: 100 });
}
