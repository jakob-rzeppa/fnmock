struct User {
    name: String,
}

#[fnmock::fakeable]
impl User {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_name_with_real_user() {
        let mut user = User { name: "Alice".into() };
        user.set_name("Bob");
        assert_eq!(user.name, "Bob");
    }

    #[test]
    fn test_set_name_with_fake_user() {
        User::set_name_fake().setup(|user, name| {
            user.name = name.to_string();
        });
        let mut user = User { name: "Alice".into() };
        user.set_name("Bob");
        assert_eq!(user.name, "Bob");
    }
}
