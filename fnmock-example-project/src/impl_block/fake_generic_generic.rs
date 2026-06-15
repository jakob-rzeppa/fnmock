use std::fmt::Display;

pub struct UserRepository<T> {
    users: Vec<T>,
}

#[fnmock::fakeable]
impl<T: Display + 'static> UserRepository<T> {
    pub fn new() -> Self {
        UserRepository { users: Vec::new() }
    }

    pub fn get_user<I: Display + 'static>(&self, user_id: I) -> Option<String> {
        Some(format!("User{}", user_id))
    }
}

fn handle_user(user_id: u32) -> String {
    let repo = UserRepository::<String>::new();
    match repo.get_user(user_id) {
        Some(user) => format!("Found: {}", user),
        None => "User not found".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use fnmock::fake;

    use super::*;

    #[test]
    fn test_handle_user_with_real_repo() {
        let result = handle_user(1);
        assert_eq!(result, "Found: User1");
    }

    #[test]
    fn test_handle_user_with_fake_repo() {
        fake!(UserRepository<String>, get_user<u32>).setup(|_, i| Some(format!("FakeUser{}", i)));
        let result = handle_user(1);
        assert_eq!(result, "Found: FakeUser1");
    }
}
