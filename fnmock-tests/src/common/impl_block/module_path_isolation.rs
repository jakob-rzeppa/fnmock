mod fake {
    //! Two `#[fnmock::fakeable]` impl blocks for same-named structs from different modules
    //! (`a::Config` vs `b::Config`), both written at the same enclosing scope,
    //! must have independent fakes and real bodies.

    mod a {
        pub struct Config;
    }

    mod b {
        pub struct Config;
    }

    #[fnmock::fakeable]
    impl a::Config {
        fn basic(&self) -> i32 {
            1
        }
    }

    #[fnmock::fakeable]
    impl b::Config {
        fn basic(&self) -> i32 {
            2
        }
    }

    #[test]
    fn test_real_bodies_are_independent() {
        assert_eq!(a::Config.basic(), 1);
        assert_eq!(b::Config.basic(), 2);
    }

    #[test]
    fn test_fakes_are_independent() {
        a::Config::basic_fake().setup(|_| 10);
        assert_eq!(a::Config.basic(), 10);
        assert_eq!(b::Config.basic(), 2);

        b::Config::basic_fake().setup(|_| 20);
        assert_eq!(a::Config.basic(), 10);
        assert_eq!(b::Config.basic(), 20);
    }
}

mod spy {
    //! Two `#[fnmock::spyable]` impl blocks for same-named structs from different modules
    //! (`a::Config` vs `b::Config`), both written at the same enclosing scope,
    //! must have independent spies.

    mod a {
        pub struct Config;
    }

    mod b {
        pub struct Config;
    }

    #[fnmock::spyable]
    impl a::Config {
        fn basic(&self) -> i32 {
            1
        }
    }

    #[fnmock::spyable]
    impl b::Config {
        fn basic(&self) -> i32 {
            2
        }
    }

    #[test]
    fn test_spies_are_independent() {
        let spy_a = a::Config::basic_spy();
        let spy_b = b::Config::basic_spy();
        spy_a.expect_once();
        spy_b.expect_once();

        assert_eq!(a::Config.basic(), 1);
        assert_eq!(b::Config.basic(), 2);

        spy_a.assert();
        spy_b.assert();
    }
}

mod mock {
    //! Two `#[fnmock::mockable]` impl blocks for same-named structs from different modules
    //! (`a::Config` vs `b::Config`), both written at the same enclosing scope,
    //! must have independent mocks.

    mod a {
        pub struct Config;
    }

    mod b {
        pub struct Config;
    }

    #[fnmock::mockable]
    impl a::Config {
        fn basic(&self) -> i32 {
            1
        }
    }

    #[fnmock::mockable]
    impl b::Config {
        fn basic(&self) -> i32 {
            2
        }
    }

    #[test]
    fn test_mocks_are_independent() {
        let mock_a = a::Config::basic_mock();
        mock_a.setup(|_| 10);
        mock_a.expect_once();

        let mock_b = b::Config::basic_mock();
        mock_b.expect_once();

        assert_eq!(a::Config.basic(), 10);
        assert_eq!(b::Config.basic(), 2);

        mock_a.assert();
        mock_b.assert();
    }
}
