#[fnmock::fakeable]
fn get_user<T, N>(id: T, name: N) -> String
    where T: std::fmt::Display + 'static, N: std::fmt::Display + 'static
{
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
