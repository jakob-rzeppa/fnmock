use std::fmt::Display;

pub struct UserRepository<T> {
    users: Vec<T>,
}

#[fnmock::fakeable]
impl<T: Display + 'static> UserRepository<T> {
    pub fn get_user<I: Display + 'static>(&self, user_id: I) -> Option<String> {
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
        UserRepository::<String>
            ::get_user_fake::<u32>()
            .setup(|_, i| Some(format!("FakeUser{}", i)));

        let repo = UserRepository::<String> { users: Vec::new() };

        let result = repo.get_user::<u32>(1);
        assert_eq!(result, Some("FakeUser1".into()));

        let result = repo.get_user::<i32>(2);
        assert_eq!(result, Some("User2".into()));

        let repo = UserRepository::<&'static str> { users: Vec::new() };

        let result = repo.get_user::<u32>(1);
        assert_eq!(result, Some("User1".into()));

        let result = repo.get_user::<i32>(2);
        assert_eq!(result, Some("User2".into()));
    }
}
