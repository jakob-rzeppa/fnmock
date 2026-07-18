//! Thread isolation for an impl-block method's fake: state set up on one thread must not be
//! visible on another. Mirror of `basic/thread_isolation.rs` for the impl-block path.

struct ThreadIsolation;

#[fnmock::fakeable]
impl ThreadIsolation {
    fn greet(&self, a: String) -> String {
        format!("Real {}", a)
    }
}

#[test]
fn test_fake_not_visible_in_spawned_thread() {
    ThreadIsolation::greet_fake().setup(|_, a| format!("Fake {}", a));

    let res = std::thread::spawn(|| ThreadIsolation.greet("Test".to_string()))
        .join()
        .unwrap();
    assert_eq!(res, "Real Test");

    // The fake set up on the main thread is still active on the main thread.
    let res = ThreadIsolation.greet("Test".to_string());
    assert_eq!(res, "Fake Test");
}

#[test]
fn test_fake_set_in_spawned_thread_does_not_leak_to_caller() {
    let res = std::thread::spawn(|| {
        ThreadIsolation::greet_fake().setup(|_, a| format!("Fake {}", a));
        ThreadIsolation.greet("Test".to_string())
    })
    .join()
    .unwrap();
    assert_eq!(res, "Fake Test");

    // The spawned thread's fake does not affect the main thread.
    let res = ThreadIsolation.greet("Test".to_string());
    assert_eq!(res, "Real Test");
}
