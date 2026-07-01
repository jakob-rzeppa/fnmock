struct User {
    name: String,
}

#[fnmock::fakeable]
impl User {
    pub fn format_user(user: Self) -> String {
        format!("Formatted user: {}", user.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_user_with_real_user() {
        let user = User { name: "Alice".into() };
        let result = User::format_user(user);
        assert_eq!(result, "Formatted user: Alice");
    }

    #[test]
    fn test_format_user_with_fake_user() {
        User::format_user_fake().setup(|user| format!("Fake formatted user: {}", user.name));
        let user = User { name: "Bob".into() };
        let result = User::format_user(user);
        assert_eq!(result, "Fake formatted user: Bob");
    }
}
