struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }
}

#[fnmock::fakeable]
impl User {
    pub fn consume(self) -> String {
        format!("Consumed user: {}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use fnmock::fake;

    use super::*;

    #[test]
    fn test_consume_with_real_user() {
        let user = User::new("Alice");
        let result = user.consume();
        assert_eq!(result, "Consumed user: Alice");
    }

    #[test]
    fn test_consume_with_fake_user() {
        fake!(User, consume).setup(|user| format!("Fake consumed user: {}", user.name));
        let user = User::new("Bob");
        let result = user.consume();
        assert_eq!(result, "Fake consumed user: Bob");
    }
}
