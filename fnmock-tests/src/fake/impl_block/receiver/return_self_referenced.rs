struct ReturnSelfReferenced {
    name: String,
}

#[fnmock::fakeable]
impl ReturnSelfReferenced {
    fn name(&self) -> &str {
        &self.name
    }
}

#[test]
fn test_return_self_referenced() {
    let s = ReturnSelfReferenced { name: "Test".to_string() };
    assert_eq!(s.name(), "Test");
}

#[test]
fn test_return_self_referenced_fake() {
    ReturnSelfReferenced::name_fake().setup(|_| "Fake");

    let s = ReturnSelfReferenced { name: "Test".to_string() };
    assert_eq!(s.name(), "Fake");
}
