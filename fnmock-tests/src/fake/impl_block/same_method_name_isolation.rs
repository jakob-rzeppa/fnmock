struct FirstStruct;

#[fnmock::fakeable]
impl FirstStruct {
    fn basic(&self) -> i32 {
        42
    }
}

struct SecondStruct;

#[fnmock::fakeable]
impl SecondStruct {
    fn basic(&self) -> i32 {
        67
    }
}

#[test]
fn test_basic() {
    let f = FirstStruct;
    assert_eq!(f.basic(), 42);

    let s = SecondStruct;
    assert_eq!(s.basic(), 67);
}

#[test]
fn test_basic_mock() {
    FirstStruct::basic_fake().setup(|_| 5);
    SecondStruct::basic_fake().setup(|_| 10);

    let f = FirstStruct;
    assert_eq!(f.basic(), 5);

    let s = SecondStruct;
    assert_eq!(s.basic(), 10);
}
