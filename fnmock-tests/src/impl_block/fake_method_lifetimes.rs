struct MyStruct {}

#[fnmock::fakeable]
impl MyStruct {
    fn with_lifetime<'a>(&self, id: &'a i32) -> String {
        format!("Result {}", id)
    }
}

#[fnmock::fakeable]
impl MyStruct {
    fn with_unused_lifetime<'a, 'b>(&self, id: &'a i32) -> String {
        format!("Result {}", id)
    }
}

#[fnmock::fakeable]
impl MyStruct {
    fn with_multiple_lifetimes<'a, 'b>(&self, id: &'a i32, name: &'b str) -> String {
        format!("Result {}: {}", id, name)
    }
}

#[fnmock::fakeable]
impl MyStruct {
    fn with_concatenated_lifetimes<'a: 'b, 'b>(&self, id: &'a i32, name: &'b str) -> String {
        format!("Result {}: {}", id, name)
    }
}

#[fnmock::fakeable]
impl MyStruct {
    fn with_concatenated_lifetimes_where<'a, 'b>(&self, id: &'a i32, name: &'b str) -> String
        where 'a: 'b
    {
        format!("Result {}: {}", id, name)
    }
}

#[fnmock::fakeable]
impl MyStruct {
    fn with_lifetime_and_generic<'a, T: std::fmt::Display + 'static>(
        &self,
        id: &'a i32,
        value: T
    ) -> String
        where T:
    {
        format!("Result {}: {}", id, value)
    }
}

#[fnmock::fakeable]
impl MyStruct {
    fn with_lifetime_and_generic_where<'a, T>(&self, id: &'a i32, value: T) -> String
        where T: std::fmt::Display + 'static
    {
        format!("Result {}: {}", id, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_lifetime_no_fake() {
        let my_struct = MyStruct {};
        let id = 42;
        let result = my_struct.with_lifetime(&id);
        assert_eq!(result, "Result 42");
    }

    #[test]
    fn test_with_lifetime_fake() {
        MyStruct::with_lifetime_fake().setup(|_, id| format!("Fake {}", id));
        let my_struct = MyStruct {};
        let id = 42;
        let result = my_struct.with_lifetime(&id);
        assert_eq!(result, "Fake 42");
    }

    #[test]
    fn test_with_unused_lifetime_no_fake() {
        let my_struct = MyStruct {};
        let id = 7;
        let result = my_struct.with_unused_lifetime(&id);
        assert_eq!(result, "Result 7");
    }

    #[test]
    fn test_with_unused_lifetime_fake() {
        MyStruct::with_unused_lifetime_fake().setup(|_, id| format!("Fake {}", id));
        let my_struct = MyStruct {};
        let id = 7;
        let result = my_struct.with_unused_lifetime(&id);
        assert_eq!(result, "Fake 7");
    }

    #[test]
    fn test_with_multiple_lifetimes_no_fake() {
        let my_struct = MyStruct {};
        let id = 11;
        let name = "Alice";
        let result = my_struct.with_multiple_lifetimes(&id, name);
        assert_eq!(result, "Result 11: Alice");
    }

    #[test]
    fn test_with_multiple_lifetimes_fake() {
        MyStruct::with_multiple_lifetimes_fake().setup(|_, id, name|
            format!("Fake {}: {}", id, name)
        );
        let my_struct = MyStruct {};
        let id = 11;
        let name = "Alice";
        let result = my_struct.with_multiple_lifetimes(&id, name);
        assert_eq!(result, "Fake 11: Alice");
    }

    #[test]
    fn test_with_concatenated_lifetimes_no_fake() {
        let my_struct = MyStruct {};
        let id = 13;
        let name = "Bob";
        let result = my_struct.with_concatenated_lifetimes(&id, name);
        assert_eq!(result, "Result 13: Bob");
    }

    #[test]
    fn test_with_concatenated_lifetimes_fake() {
        MyStruct::with_concatenated_lifetimes_fake().setup(|_, id, name|
            format!("Fake {}: {}", id, name)
        );
        let my_struct = MyStruct {};
        let id = 13;
        let name = "Bob";
        let result = my_struct.with_concatenated_lifetimes(&id, name);
        assert_eq!(result, "Fake 13: Bob");
    }

    #[test]
    fn test_with_concatenated_lifetimes_where_no_fake() {
        let my_struct = MyStruct {};
        let id = 21;
        let name = "Carol";
        let result = my_struct.with_concatenated_lifetimes_where(&id, name);
        assert_eq!(result, "Result 21: Carol");
    }

    #[test]
    fn test_with_concatenated_lifetimes_where_fake() {
        MyStruct::with_concatenated_lifetimes_where_fake().setup(|_, id, name| {
            format!("Fake {}: {}", id, name)
        });
        let my_struct = MyStruct {};
        let id = 21;
        let name = "Carol";
        let result = my_struct.with_concatenated_lifetimes_where(&id, name);
        assert_eq!(result, "Fake 21: Carol");
    }

    #[test]
    fn test_with_lifetime_and_generic_no_fake() {
        let my_struct = MyStruct {};
        let id = 34;
        let value = String::from("delta");
        let result = my_struct.with_lifetime_and_generic(&id, value);
        assert_eq!(result, "Result 34: delta");
    }

    #[test]
    fn test_with_lifetime_and_generic_fake() {
        MyStruct::with_lifetime_and_generic_fake::<String>().setup(|_, id, value|
            format!("Fake {}: {}", id, value)
        );
        let my_struct = MyStruct {};
        let id = 34;
        let value = String::from("delta");
        let result = my_struct.with_lifetime_and_generic(&id, value);
        assert_eq!(result, "Fake 34: delta");
    }

    #[test]
    fn test_with_lifetime_and_generic_where_no_fake() {
        let my_struct = MyStruct {};
        let id = 55;
        let value = String::from("echo");
        let result = my_struct.with_lifetime_and_generic_where(&id, value);
        assert_eq!(result, "Result 55: echo");
    }

    #[test]
    fn test_with_lifetime_and_generic_where_fake() {
        MyStruct::with_lifetime_and_generic_where_fake::<String>().setup(|_, id, value| {
            format!("Fake {}: {}", id, value)
        });
        let my_struct = MyStruct {};
        let id = 55;
        let value = String::from("echo");
        let result = my_struct.with_lifetime_and_generic_where(&id, value);
        assert_eq!(result, "Fake 55: echo");
    }
}
