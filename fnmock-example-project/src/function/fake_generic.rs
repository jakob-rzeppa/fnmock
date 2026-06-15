use std::{ fmt::Display };

#[fnmock::fakeable]
fn get_user<T: Display + 'static, N: Display + 'static>(id: T, name: N) -> String {
    format!("User {} ({})", id, name)
}

fn handle_user(id: i32, name: String) -> String {
    let user = get_user(id, name);
    user
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_user_no_fake() {
        let result = handle_user(1, "Alice".into());

        assert_eq!(result, "User 1 (Alice)");
    }

    #[test]
    fn test_handle_user_with_fake() {
        get_user_fake::GetUserFake::<i32, String>
            ::new()
            .setup(|id, name| format!("Fake User {} ({})", id, name));

        let result = handle_user(1, "Alice".into());

        assert_eq!(result, "Fake User 1 (Alice)");
    }
}
