trait MutDescribe {
    fn describe(&self) -> String;

    fn push_suffix(&mut self, suffix: &str);
}

impl MutDescribe for String {
    fn describe(&self) -> String {
        self.clone()
    }

    fn push_suffix(&mut self, suffix: &str) {
        self.push_str(suffix);
    }
}

#[fnmock::fakeable]
fn referenced_mut(value: &mut dyn MutDescribe) -> String {
    value.push_suffix(" Real");
    value.describe()
}

#[test]
fn test_referenced_mut() {
    let mut value = "Test".to_string();
    let result = referenced_mut(&mut value);
    assert_eq!(result, "Test Real");
}

#[test]
fn test_referenced_mut_fake() {
    referenced_mut_fake().setup(|value| {
        value.push_suffix(" Fake");
        value.describe()
    });

    let mut value = "Test".to_string();
    let result = referenced_mut(&mut value);
    assert_eq!(result, "Test Fake");
}
