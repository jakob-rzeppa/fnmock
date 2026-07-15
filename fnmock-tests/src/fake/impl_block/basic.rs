struct BasicStruct;

#[fnmock::fakeable]
impl BasicStruct {
    fn basic(&self) -> i32 {
        42
    }
}

#[test]
fn test_basic() {
    let s = BasicStruct;
    assert_eq!(s.basic(), 42);
}

#[test]
fn test_basic_mock() {
    BasicStruct::basic_fake().setup(|_| 5);

    let s = BasicStruct;
    assert_eq!(s.basic(), 5);
}
