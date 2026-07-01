struct User {
    name: String,
}

#[fnmock::fakeable]
impl User {
    pub fn write_name(&self, name: &str, buf: &mut String) {
        *buf = name.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_name_with_real_user() {
        let user = User { name: "Alice".into() };
        let mut buf = String::new();
        user.write_name("Bob", &mut buf);
        assert_eq!(buf, "Bob");
    }

    #[test]
    fn test_write_name_with_fake_user() {
        User::write_name_fake().setup(|_, name, buf| {
            *buf = name.to_string();
        });
        let user = User { name: "Alice".into() };
        let mut buf = String::new();
        user.write_name("Bob", &mut buf);
        assert_eq!(buf, "Bob");
    }
}
