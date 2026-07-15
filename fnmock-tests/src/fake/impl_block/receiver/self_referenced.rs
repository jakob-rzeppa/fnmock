struct SelfReferenced {
    value: i32,
}

#[fnmock::fakeable]
impl SelfReferenced {
    fn get(&self) -> i32 {
        self.value
    }
}

#[test]
fn test_self_referenced() {
    let s = SelfReferenced { value: 42 };
    assert_eq!(s.get(), 42);
}

#[test]
fn test_self_referenced_fake() {
    SelfReferenced::get_fake().setup(|_| 5);

    let s = SelfReferenced { value: 42 };
    assert_eq!(s.get(), 5);
}
