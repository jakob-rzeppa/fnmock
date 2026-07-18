struct TrackCallerStruct;

#[fnmock::fakeable]
impl TrackCallerStruct {
    #[track_caller]
    fn track_caller(&self) -> i32 {
        42
    }
}

#[test]
fn test_track_caller() {
    let s = TrackCallerStruct;
    assert_eq!(s.track_caller(), 42);
}

#[test]
fn test_track_caller_mock() {
    TrackCallerStruct::track_caller_fake().setup(|_| 5);

    let s = TrackCallerStruct;
    assert_eq!(s.track_caller(), 5);
}
