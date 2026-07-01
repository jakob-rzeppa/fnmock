struct User {
    name: String,
}

#[fnmock::fakeable]
impl User {
    pub fn consume(self) -> String {
        format!("Consumed user: {}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_with_real_user() {
        let user = User { name: "Alice".into() };
        let result = user.consume();
        assert_eq!(result, "Consumed user: Alice");
    }

    #[test]
    fn test_consume_with_fake_user() {
        User::consume_fake().setup(|user| format!("Fake consumed user: {}", user.name));
        let user = User { name: "Bob".into() };
        let result = user.consume();
        assert_eq!(result, "Fake consumed user: Bob");
    }
}
