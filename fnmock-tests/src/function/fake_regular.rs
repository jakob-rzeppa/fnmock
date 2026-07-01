#[fnmock::fakeable]
fn get_user(id: i32) -> String {
    format!("User {}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_no_fake() {
        let result = get_user(1);

        assert_eq!(result, "User 1");
    }

    #[test]
    fn test_get_user_with_fake() {
        get_user_fake().setup(|id| format!("Fake User {}", id));

        let result = get_user(1);

        assert_eq!(result, "Fake User 1");
    }
}
