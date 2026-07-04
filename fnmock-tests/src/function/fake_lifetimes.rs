#[fnmock::fakeable]
fn with_lifetime<'a>(a: &'a str) -> &'a str {
    a
}

#[fnmock::fakeable]
fn with_unused_lifetime<'a, 'b>(a: &'a str) -> &'a str {
    a
}

#[fnmock::fakeable]
fn with_multiple_lifetimes<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str {
    a
}

#[fnmock::fakeable]
fn with_concatenated_lifetimes<'a: 'b, 'b>(a: &'a str, _b: &'b str) -> &'b str {
    a
}

#[fnmock::fakeable]
fn with_concatenated_lifetimes_where<'a, 'b>(a: &'a str, _b: &'b str) -> &'b str where 'a: 'b {
    a
}

#[fnmock::fakeable]
fn with_lifetime_and_generic<'a, T: std::fmt::Display + 'static>(a: T, _b: T) -> T {
    a
}

#[fnmock::fakeable]
fn with_lifetime_and_generic_where<'a, T>(a: T, _b: T) -> T where T: std::fmt::Display + 'static {
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_lifetime_no_fake() {
        let a = "hello";
        let result = with_lifetime(a);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_with_lifetime_fake() {
        with_lifetime_fake().setup(|_a| "Fake");
        let a = "hello";
        let result = with_lifetime(a);
        assert_eq!(result, "Fake");
    }

    #[test]
    fn test_with_unused_lifetime_no_fake() {
        let a = "seven";
        let result = with_unused_lifetime(a);
        assert_eq!(result, "seven");
    }

    #[test]
    fn test_with_unused_lifetime_fake() {
        with_unused_lifetime_fake().setup(|_a| "Fake");
        let a = "seven";
        let result = with_unused_lifetime(a);
        assert_eq!(result, "Fake");
    }

    #[test]
    fn test_with_multiple_lifetimes_no_fake() {
        let a = "Alice";
        let b = "Bob";
        let result = with_multiple_lifetimes(a, b);
        assert_eq!(result, "Alice");
    }

    #[test]
    fn test_with_multiple_lifetimes_fake() {
        with_multiple_lifetimes_fake().setup(|_a, _b| "Fake");
        let a = "Alice";
        let b = "Bob";
        let result = with_multiple_lifetimes(a, b);
        assert_eq!(result, "Fake");
    }

    #[test]
    fn test_with_concatenated_lifetimes_no_fake() {
        let a = "A";
        let b = "B";
        let result = with_concatenated_lifetimes(a, b);
        assert_eq!(result, "A");
    }

    #[test]
    fn test_with_concatenated_lifetimes_fake() {
        with_concatenated_lifetimes_fake().setup(|_a, _b| "Fake");
        let a = "A";
        let b = "B";
        let result = with_concatenated_lifetimes(a, b);
        assert_eq!(result, "Fake");
    }

    #[test]
    fn test_with_concatenated_lifetimes_where_no_fake() {
        let a = "A";
        let b = "B";
        let result = with_concatenated_lifetimes_where(a, b);
        assert_eq!(result, "A");
    }

    #[test]
    fn test_with_concatenated_lifetimes_where_fake() {
        with_concatenated_lifetimes_where_fake().setup(|_a, _b| "Fake");
        let a = "A";
        let b = "B";
        let result = with_concatenated_lifetimes_where(a, b);
        assert_eq!(result, "Fake");
    }

    #[test]
    fn test_with_lifetime_and_generic_no_fake() {
        let a = "x";
        let value = "delta";
        let result = with_lifetime_and_generic(a, value);
        assert_eq!(result, "x");
    }

    #[test]
    fn test_with_lifetime_and_generic_fake() {
        with_lifetime_and_generic_fake::<&str>().setup(|_a, _value| "Fake");
        let a = "x";
        let value = "delta";
        let result = with_lifetime_and_generic(a, value);
        assert_eq!(result, "Fake");
    }

    #[test]
    fn test_with_lifetime_and_generic_where_no_fake() {
        let a = "x";
        let value = "echo";
        let result = with_lifetime_and_generic_where(a, value);
        assert_eq!(result, "x");
    }

    #[test]
    fn test_with_lifetime_and_generic_where_fake() {
        with_lifetime_and_generic_where_fake::<&str>().setup(|_a, _value| "Fake");
        let a = "x";
        let value = "echo";
        let result = with_lifetime_and_generic_where(a, value);
        assert_eq!(result, "Fake");
    }
}
