//! User items named like prelude items (`Option`, `Some`, `Box`, `Into`, `Fn`, `Clone`,
//! `ToString`) and user macros named like std macros (`format!`, `write!`, `thread_local!`) next to
//! the annotated item. The generated code reaches these names through `use super::*` (and the
//! macros through textual scope), so it has to spell out the std paths to stay unaffected.

#![allow(dead_code, unused_macros)]

macro_rules! format {
    ($($tokens:tt)*) => {
        compile_error!("the user's `format!` was used")
    };
}

macro_rules! write {
    ($($tokens:tt)*) => {
        compile_error!("the user's `write!` was used")
    };
}

macro_rules! thread_local {
    ($($tokens:tt)*) => {
        compile_error!("the user's `thread_local!` was used")
    };
}

struct Option;
struct Some;
struct Box;
struct Into;
struct Fn;
struct Clone;
struct ToString;

mod fake {
    // Brings the shadowing items next to the annotated items. Unused once nothing picks them up.
    #[allow(unused_imports)]
    use super::*;

    #[fnmock::fakeable]
    fn shadowed(x: u32) -> u32 {
        x + 1
    }

    #[fnmock::fakeable]
    fn shadowed_generic<T: ::std::fmt::Display + 'static>(x: T) -> u32 {
        let _ = x;
        1
    }

    struct Service;

    #[fnmock::fakeable]
    impl Service {
        fn shadowed(&self, x: u32) -> u32 {
            x + 1
        }
    }

    #[test]
    fn test_shadowed() {
        assert_eq!(shadowed(1), 2);
    }

    #[test]
    fn test_shadowed_fake() {
        shadowed_fake().setup(|x| x * 10);

        assert_eq!(shadowed(1), 10);
    }

    #[test]
    fn test_shadowed_generic_fake() {
        shadowed_generic_fake::<u32>().setup(|x| x * 10);

        assert_eq!(shadowed_generic(1u32), 10);
        assert_eq!(shadowed_generic("a"), 1);
    }

    #[test]
    fn test_shadowed_method_fake() {
        Service::shadowed_fake().setup(|_, x| x * 10);

        assert_eq!(Service.shadowed(1), 10);
    }
}

mod spy {
    // Brings the shadowing items next to the annotated items. Unused once nothing picks them up.
    #[allow(unused_imports)]
    use super::*;

    #[fnmock::spyable]
    fn shadowed(x: u32) -> u32 {
        x + 1
    }

    #[fnmock::spyable]
    fn shadowed_generic<T: ::std::fmt::Display + 'static>(x: T) -> u32 {
        let _ = x;
        1
    }

    struct Service;

    #[fnmock::spyable]
    impl Service {
        fn shadowed(&self, x: u32) -> u32 {
            x + 1
        }
    }

    #[test]
    fn test_shadowed_spy() {
        let spy = shadowed_spy();
        spy.expect(fnmock::predicate::eq(1)).once();
        spy.expect_times(1);

        assert_eq!(shadowed(1), 2);
        spy.assert();
    }

    #[test]
    #[should_panic(expected = "x == 1")]
    fn test_shadowed_spy_display() {
        let spy = shadowed_spy();
        spy.expect(fnmock::predicate::eq(1)).once();

        spy.assert();
    }

    #[test]
    fn test_shadowed_generic_spy() {
        let spy = shadowed_generic_spy::<u32>();
        spy.expect(fnmock::predicate::eq(1)).once();
        spy.expect_times(1);

        assert_eq!(shadowed_generic(1u32), 1);
        spy.assert();
    }

    #[test]
    #[should_panic(expected = "x == 1")]
    fn test_shadowed_generic_spy_display() {
        let spy = shadowed_generic_spy::<u32>();
        spy.expect(fnmock::predicate::eq(1)).once();

        spy.assert();
    }

    #[test]
    fn test_shadowed_method_spy() {
        let spy = Service::shadowed_spy();
        spy.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(Service.shadowed(1), 2);
        spy.assert();
    }
}

mod mock {
    // Brings the shadowing items next to the annotated items. Unused once nothing picks them up.
    #[allow(unused_imports)]
    use super::*;

    #[fnmock::mockable]
    fn shadowed(x: u32) -> u32 {
        x + 1
    }

    #[fnmock::mockable]
    fn shadowed_generic<T: ::std::fmt::Display + 'static>(x: T) -> u32 {
        let _ = x;
        1
    }

    struct Service;

    #[fnmock::mockable]
    impl Service {
        fn shadowed(&self, x: u32) -> u32 {
            x + 1
        }
    }

    #[test]
    fn test_shadowed_mock() {
        let mock = shadowed_mock();
        mock.setup(|x| x * 10);
        mock.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(shadowed(1), 10);
        mock.assert();
        mock.clear();
    }

    #[test]
    fn test_shadowed_generic_mock() {
        let mock = shadowed_generic_mock::<u32>();
        mock.setup(|x| x * 10);
        mock.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(shadowed_generic(1u32), 10);
        mock.assert();
        mock.clear();
    }

    #[test]
    fn test_shadowed_method_mock() {
        let mock = Service::shadowed_mock();
        mock.setup(|_, x| x * 10);
        mock.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(Service.shadowed(1), 10);
        mock.assert();
    }
}
