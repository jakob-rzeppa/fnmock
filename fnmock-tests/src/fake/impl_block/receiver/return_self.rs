#[derive(Debug, PartialEq)]
struct ReturnSelf {
    value: i32,
}

#[fnmock::fakeable]
impl ReturnSelf {
    fn doubled(&self) -> Self {
        Self { value: self.value * 2 }
    }
}

#[test]
fn test_return_self() {
    let s = ReturnSelf { value: 42 };
    assert_eq!(s.doubled(), ReturnSelf { value: 84 });
}

#[test]
fn test_return_self_fake() {
    ReturnSelf::doubled_fake().setup(|_| ReturnSelf { value: 5 });

    let s = ReturnSelf { value: 42 };
    assert_eq!(s.doubled(), ReturnSelf { value: 5 });
}
