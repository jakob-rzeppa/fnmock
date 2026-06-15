#[fnmock::fakeable]
async fn get_user(id: i32) -> String {
    format!("User {}", id)
}

async fn handle_user(id: i32) -> String {
    let user = get_user(id);
    user.await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_user_no_fake() {
        let result = handle_user(1).await;

        assert_eq!(result, "User 1");
    }

    #[tokio::test]
    async fn test_handle_user_with_fake() {
        // The closure must be not async. The result will be wrapped in a future by the `get_user` function.
        get_user_fake::GetUserFake::new().setup(|id| format!("Fake User {}", id));

        let result = handle_user(1).await;

        assert_eq!(result, "Fake User 1");
    }
}
