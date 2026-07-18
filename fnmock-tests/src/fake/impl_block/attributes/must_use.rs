struct MustUseStruct;

#[fnmock::fakeable]
impl MustUseStruct {
    #[must_use]
    fn must_use(&self) -> i32 {
        42
    }
}

#[test]
fn test_must_use() {
    let s = MustUseStruct;
    assert_eq!(s.must_use(), 42);
}

#[test]
fn test_must_use_mock() {
    MustUseStruct::must_use_fake().setup(|_| 5);

    let s = MustUseStruct;
    assert_eq!(s.must_use(), 5);
}
