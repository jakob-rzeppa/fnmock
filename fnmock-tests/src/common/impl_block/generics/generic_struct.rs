mod fake {
    struct GenericStruct<T> {
        value: T,
    }

    #[fnmock::fakeable]
    impl<T: Clone + 'static> GenericStruct<T> {
        fn get(&self) -> T {
            self.value.clone()
        }
    }

    #[test]
    fn test_generic_struct() {
        let s = GenericStruct {
            value: "Test".to_string(),
        };
        assert_eq!(s.get(), "Test");
    }

    #[test]
    fn test_generic_struct_fake() {
        GenericStruct::<String>::get_fake().setup(|_| "Fake".to_string());

        let s = GenericStruct {
            value: "Test".to_string(),
        };
        assert_eq!(s.get(), "Fake");
    }
}

mod spy {
    struct GenericStruct<T> {
        value: T,
    }

    #[fnmock::spyable]
    impl<T: Clone + 'static> GenericStruct<T> {
        fn get(&self) -> T {
            self.value.clone()
        }
    }

    #[test]
    fn test_generic_struct_spy() {
        let spy = GenericStruct::<String>::get_spy();
        spy.expect_once();

        let s = GenericStruct {
            value: "Test".to_string(),
        };
        assert_eq!(s.get(), "Test");

        spy.assert();
    }
}
