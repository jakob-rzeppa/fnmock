//! Items whose generated module names used to mangle to the same identifier:
//! - a free fn `foo__bar` next to `impl Foo { fn bar }` (both `foo__bar_…_module`),
//! - `impl FooBar` next to `impl Foo_bar` (both type names snake_cased to `foo_bar`).
//!
//! Each must get its own module, so they compile side by side and stay independent.

#![allow(non_camel_case_types, non_snake_case)]

mod fake {
    #[fnmock::fakeable]
    fn foo__bar() -> i32 {
        1
    }

    struct Foo;

    #[fnmock::fakeable]
    impl Foo {
        fn bar(&self) -> i32 {
            2
        }
    }

    struct FooBar;

    #[fnmock::fakeable]
    impl FooBar {
        fn baz(&self) -> i32 {
            3
        }
    }

    struct Foo_bar;

    #[fnmock::fakeable]
    impl Foo_bar {
        fn baz(&self) -> i32 {
            4
        }
    }

    #[test]
    fn test_real_bodies_are_independent() {
        assert_eq!(foo__bar(), 1);
        assert_eq!(Foo.bar(), 2);
        assert_eq!(FooBar.baz(), 3);
        assert_eq!(Foo_bar.baz(), 4);
    }

    #[test]
    fn test_free_fn_and_method_fakes_are_independent() {
        foo__bar_fake().setup(|| 10);
        assert_eq!(foo__bar(), 10);
        assert_eq!(Foo.bar(), 2);

        Foo::bar_fake().setup(|_| 20);
        assert_eq!(foo__bar(), 10);
        assert_eq!(Foo.bar(), 20);
    }

    #[test]
    fn test_type_name_fakes_are_independent() {
        FooBar::baz_fake().setup(|_| 30);
        assert_eq!(FooBar.baz(), 30);
        assert_eq!(Foo_bar.baz(), 4);

        Foo_bar::baz_fake().setup(|_| 40);
        assert_eq!(FooBar.baz(), 30);
        assert_eq!(Foo_bar.baz(), 40);
    }
}

mod spy {
    #[fnmock::spyable]
    fn foo__bar() -> i32 {
        1
    }

    struct Foo;

    #[fnmock::spyable]
    impl Foo {
        fn bar(&self) -> i32 {
            2
        }
    }

    struct FooBar;

    #[fnmock::spyable]
    impl FooBar {
        fn baz(&self) -> i32 {
            3
        }
    }

    struct Foo_bar;

    #[fnmock::spyable]
    impl Foo_bar {
        fn baz(&self) -> i32 {
            4
        }
    }

    #[test]
    fn test_free_fn_and_method_spies_are_independent() {
        let fn_spy = foo__bar_spy();
        let method_spy = Foo::bar_spy();
        fn_spy.expect_once();
        method_spy.expect_never();

        assert_eq!(foo__bar(), 1);

        fn_spy.assert();
        method_spy.assert();
    }

    #[test]
    fn test_type_name_spies_are_independent() {
        let foo_bar_spy = FooBar::baz_spy();
        let foo_underscore_bar_spy = Foo_bar::baz_spy();
        foo_bar_spy.expect_once();
        foo_underscore_bar_spy.expect_never();

        assert_eq!(FooBar.baz(), 3);

        foo_bar_spy.assert();
        foo_underscore_bar_spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn foo__bar() -> i32 {
        1
    }

    struct Foo;

    #[fnmock::mockable]
    impl Foo {
        fn bar(&self) -> i32 {
            2
        }
    }

    struct FooBar;

    #[fnmock::mockable]
    impl FooBar {
        fn baz(&self) -> i32 {
            3
        }
    }

    struct Foo_bar;

    #[fnmock::mockable]
    impl Foo_bar {
        fn baz(&self) -> i32 {
            4
        }
    }

    #[test]
    fn test_free_fn_and_method_mocks_are_independent() {
        let fn_mock = foo__bar_mock();
        let method_mock = Foo::bar_mock();
        fn_mock.setup(|| 10);
        fn_mock.expect_once();
        method_mock.expect_never();

        assert_eq!(foo__bar(), 10);

        fn_mock.assert();
        method_mock.assert();
    }

    #[test]
    fn test_type_name_mocks_are_independent() {
        let foo_bar_mock = FooBar::baz_mock();
        let foo_underscore_bar_mock = Foo_bar::baz_mock();
        foo_bar_mock.setup(|_| 30);
        foo_bar_mock.expect_once();
        foo_underscore_bar_mock.expect_never();

        assert_eq!(FooBar.baz(), 30);

        foo_bar_mock.assert();
        foo_underscore_bar_mock.assert();
    }
}
