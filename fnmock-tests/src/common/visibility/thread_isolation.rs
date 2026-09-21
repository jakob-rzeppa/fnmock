//! Fake and Spy state is stored in thread-local storage, so a fake set up on one thread must not be
//! visible on another thread. This pins that behavior as an intentional, documented contract.

mod fake {
    #[fnmock::fakeable]
    fn thread_isolation(a: String) -> String {
        format!("Real {}", a)
    }

    #[test]
    fn test_fake_not_visible_in_spawned_thread() {
        thread_isolation_fake().setup(|a| format!("Fake {}", a));

        let res = std::thread::spawn(|| thread_isolation("Test".to_string()))
            .join()
            .unwrap();
        assert_eq!(res, "Real Test");

        // The fake set up on the main thread is still active on the main thread.
        let res = thread_isolation("Test".to_string());
        assert_eq!(res, "Fake Test");
    }

    #[test]
    fn test_fake_set_in_spawned_thread_does_not_leak_to_caller() {
        let res = std::thread::spawn(|| {
            thread_isolation_fake().setup(|a| format!("Fake {}", a));
            thread_isolation("Test".to_string())
        })
        .join()
        .unwrap();
        assert_eq!(res, "Fake Test");

        // The spawned thread's fake does not affect the main thread.
        let res = thread_isolation("Test".to_string());
        assert_eq!(res, "Real Test");
    }
}

mod spy {
    #[fnmock::spyable]
    fn thread_isolation(a: String) -> String {
        format!("Real {}", a)
    }

    #[test]
    fn test_spy_not_visible_in_spawned_thread() {
        let spy = thread_isolation_spy();
        spy.expect_once();

        let res = std::thread::spawn(|| thread_isolation("Test".to_string()))
            .join()
            .unwrap();
        assert_eq!(res, "Real Test");

        // The spy set up on the main thread is still active on the main thread.
        let res = thread_isolation("Test".to_string());
        assert_eq!(res, "Real Test");
        spy.assert();
    }

    #[test]
    fn test_spy_set_in_spawned_thread_does_not_leak_to_caller() {
        let mutex = std::sync::Arc::new(std::sync::Mutex::new(()));
        let mutex_clone = mutex.clone();

        let guard = mutex.lock().unwrap();

        let thread_handle = std::thread::spawn(move || {
            let spy = thread_isolation_spy();
            spy.expect_once();

            let res = thread_isolation("Test".to_string());
            assert_eq!(res, "Real Test");

            let _guard = mutex_clone.lock().unwrap();
            spy.assert();
        });

        let res = thread_isolation("Test".to_string());
        assert_eq!(res, "Real Test");

        drop(guard);

        thread_handle.join().unwrap();
    }
}
