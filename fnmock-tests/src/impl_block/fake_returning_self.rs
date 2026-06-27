struct User {
    name: String,
}

#[fnmock::fakeable]
impl User {
    fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let user = User::new("Alice");
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn test_fake_new() {
        User::new_fake().setup(|name| User {
            name: format!("Fake{}", name),
        });
        let user = User::new("Bob");
        assert_eq!(user.name, "FakeBob");
    }
}
