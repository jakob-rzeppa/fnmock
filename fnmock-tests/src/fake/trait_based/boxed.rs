trait Describe {
    fn describe(&self) -> String;
}

impl Describe for String {
    fn describe(&self) -> String {
        self.clone()
    }
}

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
