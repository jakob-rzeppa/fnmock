use mock_lib::derive::use_function_mock;

#[use_function_mock]
use crate::two_functions::service::{fetch_user, send_email};

mod service;


/// Business logic that uses both functions
pub fn notify_user(user_id: u32, subject: String) -> Result<(), String> {
    let user = fetch_user(user_id)?;
    send_email(user, subject)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use service::fetch_user_mock;
    use service::send_email_mock;

    #[test]
    fn test_notify_user_success() {
        // Mock fetch_user to return a test user
        fetch_user_mock::mock_implementation(|_id| {
            Ok("test_user@example.com".to_string())
        });

        // Mock send_email to succeed without actually sending
        send_email_mock::mock_implementation(|(_to, _subject)| {
            Ok(())
        });

        // Test the business logic
        let result = notify_user(1, "Welcome!".to_string());

        assert!(result.is_ok());

        // Verify both functions were called once
        fetch_user_mock::assert_times(1);
        send_email_mock::assert_times(1);

        // Clean up mocks
        fetch_user_mock::clear_mock();
        send_email_mock::clear_mock();
    }

    #[test]
    fn test_notify_user_fetch_fails() {
        // Mock fetch_user to fail
        fetch_user_mock::mock_implementation(|_id| {
            Err("Database error".to_string())
        });

        // send_email should not be called since fetch_user fails
        send_email_mock::mock_implementation(|(_to, _subject)| {
            panic!("Should not be called!");
        });

        // Test the business logic
        let result = notify_user(1, "Welcome!".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Database error");

        // Verify fetch_user was called but send_email was not
        fetch_user_mock::assert_times(1);
        send_email_mock::assert_times(0);

        // Clean up mocks
        fetch_user_mock::clear_mock();
        send_email_mock::clear_mock();
    }

    #[test]
    fn test_with_specific_params() {
        // Mock with passthrough
        fetch_user_mock::mock_implementation(|id| {
            Ok(format!("user_{}@test.com", id))
        });

        send_email_mock::mock_implementation(|(_to, _subject)| {
            Ok(())
        });

        // Call with specific user ID
        let _ = notify_user(42, "Test Subject".to_string());

        // Verify the exact parameters
        fetch_user_mock::assert_with(42);
        send_email_mock::assert_with(("user_42@test.com".to_string(), "Test Subject".to_string()));

        // Clean up
        fetch_user_mock::clear_mock();
        send_email_mock::clear_mock();
    }
}
