#[fnmock::fakeable]
fn get_user() -> String {
    "Alice".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_original() {
        // Call the original function and assert the return value
        let result = get_user();
        assert_eq!(result, "Alice");
    }

    #[test]
    fn test_get_user_fake() {
        // Set up the fake implementation for get_user
        get_user_fake().setup(|| "Bob".to_string());

        // Call the mocked function and assert the return value
        let result = get_user();
        assert_eq!(result, "Bob");
    }
}
