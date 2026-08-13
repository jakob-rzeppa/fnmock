//! The per-instantiation stores live behind the same `thread_local!` as the
//! plain spy store, so a call made on another thread is invisible here.

#[fnmock::spyable]
fn thread_isolation<T: 'static>(a: T) {
    let _ = a;
}

#[test]
fn test_calls_on_another_thread_are_not_recorded_here() {
    let spy = thread_isolation_spy::<i32>();
    spy.expect(fnmock::predicate::always()).once();

    std::thread::spawn(|| {
        thread_isolation(1);
        thread_isolation(2);
    })
    .join()
    .unwrap();

    thread_isolation(3);

    spy.assert();
}

#[test]
fn test_expectations_do_not_leak_into_a_spawned_thread() {
    let spy = thread_isolation_spy::<i32>();
    spy.expect_never();

    // The spawned thread has its own empty store, so this must not trip the
    // `expect_never` set above.
    std::thread::spawn(|| {
        thread_isolation(1);
    })
    .join()
    .unwrap();

    spy.assert();
}
