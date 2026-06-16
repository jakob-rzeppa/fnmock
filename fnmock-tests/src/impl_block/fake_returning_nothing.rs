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
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

#[cfg(test)]
mod tests {
    use fnmock::fake;

    use super::*;

    #[test]
    fn test_set_name_with_real_user() {
        let mut user = User::new("Alice");
        user.set_name("Bob");
        assert_eq!(user.name, "Bob");
    }

    #[test]
    fn test_set_name_with_fake_user() {
        fake!(User, set_name).setup(|user, name| {
            user.name = name.to_string();
        });
        let mut user = User::new("Alice");
        user.set_name("Bob");
        assert_eq!(user.name, "Bob");
    }
}
