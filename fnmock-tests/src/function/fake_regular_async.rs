#[fnmock::fakeable]
async fn get_user(id: i32) -> String {
    format!("User {}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_no_fake() {
        let result = get_user(1).await;

        assert_eq!(result, "User 1");
    }

    #[tokio::test]
    async fn test_get_user_with_fake() {
        // The closure must be not async. The result will be wrapped in a future by the `get_user` function.
        get_user_fake().setup(|id| format!("Fake User {}", id));

        let result = get_user(1).await;

        assert_eq!(result, "Fake User 1");
    }
}
