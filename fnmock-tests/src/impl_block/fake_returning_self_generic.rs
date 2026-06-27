struct User<T> {
    name: T,
}

#[fnmock::fakeable]
impl<T: 'static> User<T> {
    fn new(name: T) -> Self {
        User {
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let user = User::new("Alice".to_string());
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn test_fake_new() {
        User::new_fake().setup(|name| User {
            name: format!("Fake{}", name),
        });
        let user = User::new("Bob".to_string());
        assert_eq!(user.name, "FakeBob");
    }
}
