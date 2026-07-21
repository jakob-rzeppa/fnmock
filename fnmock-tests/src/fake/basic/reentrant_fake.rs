//! A fake closure that calls back into its own accessor (`is_set`, `setup`, `clear`) must not
//! panic. The generated `is_set`/`get` both scope their `RefCell` borrow to a single
//! `store.with(|store| ...)` call and hand back an owned value (a `bool`, or a cloned `Rc`), so the
//! borrow is released before the fake closure itself ever runs.

#[fnmock::fakeable]
fn reentrant_fake(a: i32) -> i32 {
    a + 1
}

#[test]
fn test_is_set_inside_fake_closure_does_not_panic() {
    reentrant_fake_fake().setup(|a| {
        assert!(reentrant_fake_fake().is_set());
        a + 100
    });

    assert_eq!(reentrant_fake(1), 101);
}

#[test]
fn test_clear_inside_fake_closure_does_not_panic() {
    reentrant_fake_fake().setup(|a| {
        reentrant_fake_fake().clear();
        a + 100
    });

    // The closure that was active when the call started still runs to completion...
    assert_eq!(reentrant_fake(1), 101);
    // ...but it cleared itself, so the next call runs the real body.
    assert_eq!(reentrant_fake(1), 2);
}

#[test]
fn test_setup_inside_fake_closure_does_not_panic() {
    reentrant_fake_fake().setup(|a| {
        reentrant_fake_fake().setup(|a| a + 999);
        a + 100
    });

    // The inline call clones the implementation out before invoking it, so the in-flight call
    // keeps running the closure that was installed when it started...
    assert_eq!(reentrant_fake(1), 101);
    // ...but the re-entrant `setup` call installed a new fake, visible on the next call.
    assert_eq!(reentrant_fake(1), 1000);
}
