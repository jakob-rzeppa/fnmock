use std::fmt::Display;
use std::marker::PhantomData;

pub struct StructWithLifetime<'a> {
    label: &'a str,
}

#[fnmock::fakeable]
impl<'a> StructWithLifetime<'a> {
    pub fn with_lifetime(&self, id: i32) -> String {
        format!("Result {}: {}", self.label, id)
    }
}

pub struct StructWithUnusedLifetime<'a, 'b> {
    label: &'a str,
    _marker: PhantomData<&'b ()>,
}

#[fnmock::fakeable]
impl<'a, 'b> StructWithUnusedLifetime<'a, 'b> {
    pub fn with_unused_lifetime(&self, id: i32) -> String {
        format!("Result {}: {}", self.label, id)
    }
}

pub struct StructWithMultipleLifetimes<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

#[fnmock::fakeable]
impl<'a, 'b> StructWithMultipleLifetimes<'a, 'b> {
    pub fn with_multiple_lifetimes(&self, id: i32) -> String {
        format!("Result {}: {}: {}", self.first, self.second, id)
    }
}

pub struct StructWithConcatenatedLifetimes<'a: 'b, 'b> {
    first: &'a str,
    second: &'b str,
}

#[fnmock::fakeable]
impl<'a: 'b, 'b> StructWithConcatenatedLifetimes<'a, 'b> {
    pub fn with_concatenated_lifetimes(&self, id: i32) -> String {
        format!("Result {}: {}: {}", self.first, self.second, id)
    }
}

pub struct StructWithConcatenatedLifetimesWhere<'a, 'b> where 'a: 'b {
    first: &'a str,
    second: &'b str,
}

#[fnmock::fakeable]
impl<'a, 'b> StructWithConcatenatedLifetimesWhere<'a, 'b> where 'a: 'b {
    pub fn with_concatenated_lifetimes_where(&self, id: i32) -> String {
        format!("Result {}: {}: {}", self.first, self.second, id)
    }
}

pub struct StructWithLifetimeAndGeneric<'a, T: Display + 'static> {
    label: &'a str,
    value: T,
}

#[fnmock::fakeable]
impl<'a, T: Display + 'static> StructWithLifetimeAndGeneric<'a, T> {
    pub fn with_lifetime_and_generic(&self, id: i32) -> String {
        format!("Result {}: {}: {}", self.label, self.value, id)
    }
}

pub struct StructWithLifetimeAndGenericWhere<'a, T> where T: Display + 'static {
    label: &'a str,
    value: T,
}

#[fnmock::fakeable]
impl<'a, T> StructWithLifetimeAndGenericWhere<'a, T> where T: Display + 'static {
    pub fn with_lifetime_and_generic_where(&self, id: i32) -> String {
        format!("Result {}: {}: {}", self.label, self.value, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_lifetime_no_fake() {
        let my_struct = StructWithLifetime { label: "Alpha" };
        let result = my_struct.with_lifetime(42);
        assert_eq!(result, "Result Alpha: 42");
    }

    #[test]
    fn test_with_lifetime_fake() {
        StructWithLifetime::with_lifetime_fake().setup(|my_struct, id| {
            format!("Fake {}: {}", my_struct.label, id)
        });
        let my_struct = StructWithLifetime { label: "Alpha" };
        let result = my_struct.with_lifetime(42);
        assert_eq!(result, "Fake Alpha: 42");
    }

    #[test]
    fn test_with_unused_lifetime_no_fake() {
        let my_struct = StructWithUnusedLifetime {
            label: "Beta",
            _marker: PhantomData,
        };
        let result = my_struct.with_unused_lifetime(7);
        assert_eq!(result, "Result Beta: 7");
    }

    #[test]
    fn test_with_unused_lifetime_fake() {
        StructWithUnusedLifetime::with_unused_lifetime_fake().setup(|my_struct, id| {
            format!("Fake {}: {}", my_struct.label, id)
        });
        let my_struct = StructWithUnusedLifetime {
            label: "Beta",
            _marker: PhantomData,
        };
        let result = my_struct.with_unused_lifetime(7);
        assert_eq!(result, "Fake Beta: 7");
    }

    #[test]
    fn test_with_multiple_lifetimes_no_fake() {
        let my_struct = StructWithMultipleLifetimes {
            first: "Alice",
            second: "Johnson",
        };
        let result = my_struct.with_multiple_lifetimes(11);
        assert_eq!(result, "Result Alice: Johnson: 11");
    }

    #[test]
    fn test_with_multiple_lifetimes_fake() {
        StructWithMultipleLifetimes::with_multiple_lifetimes_fake().setup(|my_struct, id| {
            format!("Fake {}: {}: {}", my_struct.first, my_struct.second, id)
        });
        let my_struct = StructWithMultipleLifetimes {
            first: "Alice",
            second: "Johnson",
        };
        let result = my_struct.with_multiple_lifetimes(11);
        assert_eq!(result, "Fake Alice: Johnson: 11");
    }

    #[test]
    fn test_with_concatenated_lifetimes_no_fake() {
        let my_struct = StructWithConcatenatedLifetimes {
            first: "Bob",
            second: "Builder",
        };
        let result = my_struct.with_concatenated_lifetimes(13);
        assert_eq!(result, "Result Bob: Builder: 13");
    }

    #[test]
    fn test_with_concatenated_lifetimes_fake() {
        StructWithConcatenatedLifetimes::with_concatenated_lifetimes_fake().setup(|my_struct, id| {
            format!("Fake {}: {}: {}", my_struct.first, my_struct.second, id)
        });
        let my_struct = StructWithConcatenatedLifetimes {
            first: "Bob",
            second: "Builder",
        };
        let result = my_struct.with_concatenated_lifetimes(13);
        assert_eq!(result, "Fake Bob: Builder: 13");
    }

    #[test]
    fn test_with_concatenated_lifetimes_where_no_fake() {
        let my_struct = StructWithConcatenatedLifetimesWhere {
            first: "Carol",
            second: "Clark",
        };
        let result = my_struct.with_concatenated_lifetimes_where(21);
        assert_eq!(result, "Result Carol: Clark: 21");
    }

    #[test]
    fn test_with_concatenated_lifetimes_where_fake() {
        StructWithConcatenatedLifetimesWhere::with_concatenated_lifetimes_where_fake().setup(
            |my_struct, id| format!("Fake {}: {}: {}", my_struct.first, my_struct.second, id)
        );
        let my_struct = StructWithConcatenatedLifetimesWhere {
            first: "Carol",
            second: "Clark",
        };
        let result = my_struct.with_concatenated_lifetimes_where(21);
        assert_eq!(result, "Fake Carol: Clark: 21");
    }

    #[test]
    fn test_with_lifetime_and_generic_no_fake() {
        let my_struct = StructWithLifetimeAndGeneric {
            label: "Delta",
            value: String::from("payload"),
        };
        let result = my_struct.with_lifetime_and_generic(34);
        assert_eq!(result, "Result Delta: payload: 34");
    }

    #[test]
    fn test_with_lifetime_and_generic_fake() {
        StructWithLifetimeAndGeneric::<String>
            ::with_lifetime_and_generic_fake()
            .setup(|my_struct, id|
                format!("Fake {}: {}: {}", my_struct.label, my_struct.value, id)
            );
        let my_struct = StructWithLifetimeAndGeneric {
            label: "Delta",
            value: String::from("payload"),
        };
        let result = my_struct.with_lifetime_and_generic(34);
        assert_eq!(result, "Fake Delta: payload: 34");
    }

    #[test]
    fn test_with_lifetime_and_generic_where_no_fake() {
        let my_struct = StructWithLifetimeAndGenericWhere {
            label: "Echo",
            value: String::from("signal"),
        };
        let result = my_struct.with_lifetime_and_generic_where(55);
        assert_eq!(result, "Result Echo: signal: 55");
    }

    #[test]
    fn test_with_lifetime_and_generic_where_fake() {
        StructWithLifetimeAndGenericWhere::<String>
            ::with_lifetime_and_generic_where_fake()
            .setup(|my_struct, id| {
                format!("Fake {}: {}: {}", my_struct.label, my_struct.value, id)
            });
        let my_struct = StructWithLifetimeAndGenericWhere {
            label: "Echo",
            value: String::from("signal"),
        };
        let result = my_struct.with_lifetime_and_generic_where(55);
        assert_eq!(result, "Fake Echo: signal: 55");
    }
}
