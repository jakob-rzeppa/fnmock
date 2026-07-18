struct DeprecatedStruct;

#[fnmock::fakeable]
impl DeprecatedStruct {
    #[deprecated]
    fn deprecated(&self) -> i32 {
        42
    }
}

#[test]
fn test_deprecated() {
    let s = DeprecatedStruct;
    #[allow(deprecated)]
    let res = s.deprecated();
    assert_eq!(res, 42);
}

#[test]
fn test_deprecated_mock() {
    DeprecatedStruct::deprecated_fake().setup(|_| 5);

    let s = DeprecatedStruct;
    #[allow(deprecated)]
    let res = s.deprecated();
    assert_eq!(res, 5);
}
