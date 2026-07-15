#[cfg(test)]
#[fnmock::fakeable]
fn cfg_test(a: String) -> String {
    a
}

fn normal_function() {
    // Uncomment the following line to see, cfg_test is not available in non-test builds.
    // cfg_test("Test".to_string());
}

// Tests are by default only compiled in test builds, so the cfg_test function and fake are available here.
#[test]
fn test_cfg_test() {
    let res = cfg_test("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_cfg_test_fake() {
    cfg_test_fake().setup(|a| format!("Fake {}", a));
    let res = cfg_test("Test".to_string());
    assert_eq!(res, "Fake Test");
}
