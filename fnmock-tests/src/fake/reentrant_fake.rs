//! A fake closure that calls back into its own accessor (`is_set`, `setup`, `clear`).

mod fake {
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
}

mod mock {
    #[fnmock::mockable]
    fn reentrant_fake(a: i32) -> i32 {
        a + 1
    }

    #[test]
    fn test_setup_inside_mock_closure_still_records_the_call() {
        let mock = reentrant_fake_mock();
        mock.setup(|a| {
            reentrant_fake_mock().setup(|a| a + 999);
            a + 100
        });
        mock.expect_times(2);

        assert_eq!(reentrant_fake(1), 101);
        assert_eq!(reentrant_fake(1), 1000);
        mock.assert();
    }
}
