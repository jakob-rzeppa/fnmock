async fn get_user(id: i32) -> String {
    format!("User {}", id)
}

async fn handle_user(id: i32) -> String {
    let user = get_user(id).await;
    user
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
        let result = handle_user(1).await;

        assert_eq!(result, "Fake User 1");
    }
}
