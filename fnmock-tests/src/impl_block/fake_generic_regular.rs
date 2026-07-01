pub struct UserRepository<T> {
    users: Vec<T>,
}

#[fnmock::fakeable]
impl<T: 'static> UserRepository<T> {
    pub fn get_user(&self, user_id: u32) -> Option<String> {
        Some(format!("User{}", user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_with_real_repo() {
        let repo = UserRepository::<String> { users: Vec::new() };
        let result = repo.get_user(1);
        assert_eq!(result, Some("User1".into()));
    }

    #[test]
    fn test_get_user_with_fake_repo() {
        UserRepository::<String>::get_user_fake().setup(|_, i| Some(format!("FakeUser{}", i)));

        let repo = UserRepository::<String> { users: Vec::new() };
        let result = repo.get_user(1);
        assert_eq!(result, Some("FakeUser1".into()));
    }
}
