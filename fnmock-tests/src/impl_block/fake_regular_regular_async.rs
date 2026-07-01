pub struct UserRepository;

#[fnmock::fakeable]
impl UserRepository {
    pub async fn get_user(&self, user_id: u32) -> Option<String> {
        Some(format!("User{}", user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_with_real_repo() {
        let repo = UserRepository;
        let result = repo.get_user(1).await;
        assert_eq!(result, Some("User1".into()));
    }

    #[tokio::test]
    async fn test_get_user_with_fake_repo() {
        UserRepository::get_user_fake().setup(|_, i| {
            if i == 1 { Some(format!("FakeUser{}", i)) } else { None }
        });

        let repo = UserRepository;
        let result = repo.get_user(1).await;
        assert_eq!(result, Some("FakeUser1".into()));

        let result = repo.get_user(2).await;
        assert_eq!(result, None);
    }
}
