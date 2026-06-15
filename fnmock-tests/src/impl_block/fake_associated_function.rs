struct User {
    name: String,
}

#[fnmock::fakeable]
impl User {
    pub fn default() -> Self {
        User {
            name: "Default".to_string(),
        }
    }

    pub fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use fnmock::fake;

    use super::*;

    #[test]
    fn test_default() {
        let user = User::default();
        assert_eq!(user.name, "Default");
    }

    #[test]
    fn test_new() {
        let user = User::new("Alice");
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn test_fake_default() {
        fake!(User, default).setup(|| User {
            name: "FakeDefault".to_string(),
        });
        let user = User::default();
        assert_eq!(user.name, "FakeDefault");
    }

    #[test]
    fn test_fake_new() {
        fake!(User, new).setup(|name| User {
            name: format!("Fake{}", name),
        });
        let user = User::new("Bob");
        assert_eq!(user.name, "FakeBob");
    }
}
