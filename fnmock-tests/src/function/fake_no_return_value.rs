#[fnmock::fakeable]
fn append_to_string(original: &mut String, new: &str) {
    original.push_str(new);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_to_string_original() {
        let mut original = "Hello, ".to_string();
        append_to_string(&mut original, "World!");
        assert_eq!(original, "Hello, World!");
    }

    #[test]
    fn test_append_to_string_fake() {
        // Set up the fake implementation for append_to_string
        append_to_string_fake().setup(|original, new| {
            original.push_str(new);
        });

        // Call the mocked function and assert the return value
        let mut original = "Hello, ".to_string();
        append_to_string(&mut original, "World!");
        assert_eq!(original, "Hello, World!");
    }
}
