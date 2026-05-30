fn get_user(id: i32) -> String {
    format!("User {}", id)
}

fn handle_user(id: i32) -> String {
    let user = get_user(id);
    user
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_user_no_fake() {
        let result = handle_user(1);

        assert_eq!(result, "User 1");
    }

    #[test]
    fn test_handle_user_with_fake() {
        let result = handle_user(1);

        assert_eq!(result, "Fake User 1");
    }
}
