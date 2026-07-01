use std::{ fmt::Display };

#[fnmock::fakeable]
fn get_user<T: Display + 'static, N: Display + 'static>(id: T, name: N) -> String {
    format!("User {} ({})", id, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_no_fake() {
        let result = get_user::<i32, String>(1, "Alice".into());

        assert_eq!(result, "User 1 (Alice)");
    }

    #[test]
    fn test_get_user_with_fake() {
        get_user_fake::<i32, String>().setup(|id, name| format!("Fake User {} ({})", id, name));

        let result = get_user::<i32, String>(1, "Alice".into());

        assert_eq!(result, "Fake User 1 (Alice)");
    }
}
