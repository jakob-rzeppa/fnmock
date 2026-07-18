//! Generic fake state lives in the `GenericFakeStore`'s own thread-local static, so a fake
//! set up on one thread must not be visible on another. Mirror of `basic/thread_isolation.rs`
//! for the generic store path.

#[fnmock::fakeable]
fn generic_thread_isolation<T: 'static + std::fmt::Display>(a: T) -> String {
    format!("Real {}", a)
}

#[test]
fn test_fake_not_visible_in_spawned_thread() {
    generic_thread_isolation_fake::<String>().setup(|a| format!("Fake {}", a));

    let res = std::thread::spawn(|| generic_thread_isolation("Test".to_string()))
        .join()
        .unwrap();
    assert_eq!(res, "Real Test");

    // The fake set up on the main thread is still active on the main thread.
    let res = generic_thread_isolation("Test".to_string());
    assert_eq!(res, "Fake Test");
}

#[test]
fn test_fake_set_in_spawned_thread_does_not_leak_to_caller() {
    let res = std::thread::spawn(|| {
        generic_thread_isolation_fake::<String>().setup(|a| format!("Fake {}", a));
        generic_thread_isolation("Test".to_string())
    })
    .join()
    .unwrap();
    assert_eq!(res, "Fake Test");

    // The spawned thread's fake does not affect the main thread.
    let res = generic_thread_isolation("Test".to_string());
    assert_eq!(res, "Real Test");
}
