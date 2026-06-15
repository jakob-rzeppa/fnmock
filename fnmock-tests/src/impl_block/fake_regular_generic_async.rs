use std::fmt::Display;

pub struct UserRepository;

#[fnmock::fakeable]
impl UserRepository {
    pub async fn get_user<T: Display + 'static>(&self, user_id: T) -> Option<String> {
        Some(format!("User{}", user_id))
    }
}

async fn handle_user(user_id: u32) -> String {
    let repo = UserRepository;
    match repo.get_user(user_id).await {
        Some(user) => format!("Found: {}", user),
        None => "User not found".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use fnmock::fake;

    use super::*;

    #[tokio::test]
    async fn test_handle_user_with_real_repo() {
        let result = handle_user(1).await;
        assert_eq!(result, "Found: User1");
    }

    #[tokio::test]
    async fn test_handle_user_with_fake_repo() {
        fake!(UserRepository, get_user<u32>).setup(|_, i| {
            if i == 1 { Some(format!("FakeUser{}", i)) } else { None }
        });

        let result = handle_user(1).await;
        assert_eq!(result, "Found: FakeUser1");

        let result = handle_user(2).await;
        assert_eq!(result, "User not found");
    }
}
