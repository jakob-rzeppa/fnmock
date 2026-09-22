trait Describe {
    fn describe(&self) -> String;
}

impl Describe for String {
    fn describe(&self) -> String {
        self.clone()
    }
}

mod fake {
    use crate::common::traits::boxed::Describe;

    #[fnmock::fakeable]
    fn boxed(value: Box<dyn Describe>) -> String {
        format!("Real {}", value.describe())
    }

    #[test]
    fn test_boxed() {
        let result = boxed(Box::new("Test".to_string()));
        assert_eq!(result, "Real Test");
    }

    #[test]
    fn test_boxed_fake() {
        boxed_fake().setup(|value| format!("Fake {}", value.describe()));

        let result = boxed(Box::new("Test".to_string()));
        assert_eq!(result, "Fake Test");
    }
}

mod spy {
    use crate::common::traits::boxed::Describe;

    #[fnmock::spyable]
    fn boxed(value: Box<dyn Describe>) -> String {
        format!("Real {}", value.describe())
    }

    #[test]
    fn test_boxed_spy() {
        let spy = boxed_spy();
        spy.expectf(|d| d.describe() == "Test");

        let result = boxed(Box::new("Test".to_string()));
        assert_eq!(result, "Real Test");

        spy.assert();
    }
}

mod mock {
    use crate::common::traits::boxed::Describe;

    #[fnmock::mockable]
    fn boxed(value: Box<dyn Describe>) -> String {
        format!("Real {}", value.describe())
    }

    #[test]
    fn test_boxed() {
        let mock = boxed_mock();
        mock.setup(|value| format!("Fake {}", value.describe()));
        mock.expectf(|d| d.describe() == "Test").once();

        let result = boxed(Box::new("Test".to_string()));

        assert_eq!(result, "Fake Test");
        mock.assert();
    }
}
