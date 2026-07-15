struct CfgStruct;

#[fnmock::fakeable]
impl CfgStruct {
    #[cfg(test)]
    fn cfg(&self) -> i32 {
        42
    }
}

#[test]
fn test_cfg() {
    let s = CfgStruct;
    assert_eq!(s.cfg(), 42);
}

#[test]
fn test_cfg_mock() {
    CfgStruct::cfg_fake().setup(|_| 5);

    let s = CfgStruct;
    assert_eq!(s.cfg(), 5);
}
