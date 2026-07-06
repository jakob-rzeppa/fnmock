#[track_caller]
#[fnmock::fakeable]
fn track_caller(a: String) -> String {
    a
}

// track_caller does not interfere with our fake implementation.

#[test]
fn test_track_caller() {
    let res = track_caller("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_track_caller_fake() {
    track_caller_fake().setup(|a| format!("Fake {}", a));
    let res = track_caller("Test".to_string());
    assert_eq!(res, "Fake Test");
}
