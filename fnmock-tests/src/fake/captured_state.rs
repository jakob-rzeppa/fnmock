//! What a fake closure captured has to survive across calls. The generated inline call clones
//! the stored implementation out of the `RefCell` and invokes the clone, so every call runs the
//! *same* closure rather than a fresh one, and captured state accumulates instead of resetting.

mod fake {
    #[fnmock::fakeable]
    fn captured_state(a: i32) -> i32 {
        a
    }

    #[test]
    fn test_captured_state_persists_across_calls() {
        use std::{cell::Cell, rc::Rc};

        let calls = Rc::new(Cell::new(0));

        let seen = Rc::clone(&calls);
        captured_state_fake().setup(move |a| {
            seen.set(seen.get() + 1);
            a + seen.get()
        });

        assert_eq!(captured_state(10), 11);
        assert_eq!(captured_state(10), 12);
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn test_clear_drops_the_closure_and_what_it_captured() {
        use std::{cell::Cell, rc::Rc};

        let calls = Rc::new(Cell::new(0));

        let seen = Rc::clone(&calls);
        captured_state_fake().setup(move |a| {
            seen.set(seen.get() + 1);
            a
        });

        captured_state(1);
        captured_state_fake().clear();
        captured_state(1);

        // The real body ran the second time, so the closure never saw that call...
        assert_eq!(calls.get(), 1);
        // ...and `clear` dropped the closure, leaving us the only owner of the captured `Rc`.
        assert_eq!(Rc::strong_count(&calls), 1);
    }

    #[test]
    fn test_setup_twice_drops_what_the_replaced_closure_captured() {
        use std::{cell::Cell, rc::Rc};

        let calls = Rc::new(Cell::new(0));

        let seen = Rc::clone(&calls);
        captured_state_fake().setup(move |a| {
            seen.set(seen.get() + 1);
            a
        });
        captured_state_fake().setup(|a| a);

        assert_eq!(Rc::strong_count(&calls), 1);
    }

    #[test]
    #[should_panic(expected = "boom")]
    fn test_a_panic_from_the_fake_closure_propagates_to_the_caller() {
        captured_state_fake().setup(|_| panic!("boom"));

        captured_state(1);
    }
}

mod mock {
    #[fnmock::mockable]
    fn captured_state(a: i32) -> i32 {
        a
    }

    #[test]
    fn test_captured_state_persists_across_calls() {
        use std::{cell::Cell, rc::Rc};

        let calls = Rc::new(Cell::new(0));

        let mock = captured_state_mock();
        let seen = Rc::clone(&calls);
        mock.setup(move |a| {
            seen.set(seen.get() + 1);
            a + seen.get()
        });
        mock.expect_times(2);

        assert_eq!(captured_state(10), 11);
        assert_eq!(captured_state(10), 12);
        assert_eq!(calls.get(), 2);
        mock.assert();
    }
}
