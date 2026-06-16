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
    pub fn write_name(&self, name: &str, buf: &mut String) {
        *buf = name.to_string();
    }
}

#[cfg(test)]
mod tests {
    use fnmock::fake;

    use super::*;

    #[test]
    fn test_write_name_with_real_user() {
        let user = User::new("Alice");
        let mut buf = String::new();
        user.write_name("Bob", &mut buf);
        assert_eq!(buf, "Bob");
    }

    #[test]
    fn test_write_name_with_fake_user() {
        fake!(User, write_name).setup(|_, name, buf| {
            *buf = name.to_string();
        });
        let user = User::new("Alice");
        let mut buf = String::new();
        user.write_name("Bob", &mut buf);
        assert_eq!(buf, "Bob");
    }
}
