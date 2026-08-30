trait Describe {
    fn describe(&self) -> String;
}

impl Describe for String {
    fn describe(&self) -> String {
        self.clone()
    }
}

mod fake {
    use crate::common::traits::referenced::Describe;

    #[fnmock::fakeable]
    fn referenced(value: &dyn Describe) -> String {
        format!("Real {}", value.describe())
    }

    #[test]
    fn test_referenced() {
        let value = "Test".to_string();
        let result = referenced(&value);
        assert_eq!(result, "Real Test");
    }

    #[test]
    fn test_referenced_fake() {
        referenced_fake().setup(|value| format!("Fake {}", value.describe()));

        let value = "Test".to_string();
        let result = referenced(&value);
        assert_eq!(result, "Fake Test");
    }
}

mod spy {
    use crate::common::traits::referenced::Describe;

    #[fnmock::spyable]
    fn referenced(value: &dyn Describe) -> String {
        format!("Real {}", value.describe())
    }

    #[test]
    fn test_referenced_spy() {
        let spy = referenced_spy();
        let value = "Test".to_string();
        spy.expectf(|d| d.describe() == "Test");

        let result = referenced(&value);
        assert_eq!(result, "Real Test");

        spy.assert();
    }
}
