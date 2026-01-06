use crate::two_functions::service::fetch_user;

mod service;


/// Business logic that uses both functions
pub fn notify_user(user_id: u32, subject: String) -> Result<(), String> {
    let user = fetch_user(user_id)?;
    // send_email(user, subject)?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use service::mock;
//
//     #[test]
//     fn test_notify_user_success() {
//         // Mock fetch_user to return a test user
//         mock::fetch_user::mock_implementation(|_id| {
//             Ok("test_user@example.com".to_string())
//         });
//
//         // Mock send_email to succeed without actually sending
//         mock::send_email::mock_implementation(|_to, _subject| {
//             Ok(())
//         });
//
//         // Test the business logic
//         let result = mock::notify_user(1, "Welcome!".to_string());
//
//         assert!(result.is_ok());
//
//         // Verify both functions were called once
//         mock::fetch_user::assert_times(1);
//         mock::send_email::assert_times(1);
//
//         // Clean up mocks
//         mock::fetch_user::clear_mock();
//         mock::send_email::clear_mock();
//     }
//
//     #[test]
//     fn test_notify_user_fetch_fails() {
//         // Mock fetch_user to fail
//         mock::fetch_user::mock_implementation(|_id| {
//             Err("Database error".to_string())
//         });
//
//         // send_email should not be called since fetch_user fails
//         mock::send_email::mock_implementation(|_to, _subject| {
//             panic!("Should not be called!");
//         });
//
//         // Test the business logic
//         let result = mock::notify_user(1, "Welcome!".to_string());
//
//         assert!(result.is_err());
//         assert_eq!(result.unwrap_err(), "Database error");
//
//         // Verify fetch_user was called but send_email was not
//         mock::fetch_user::assert_times(1);
//         mock::send_email::assert_times(0);
//
//         // Clean up mocks
//         mock::fetch_user::clear_mock();
//         mock::send_email::clear_mock();
//     }
//
//     #[test]
//     fn test_with_specific_params() {
//         // Mock with passthrough
//         mock::fetch_user::mock_implementation(|id| {
//             Ok(format!("user_{}@test.com", id))
//         });
//
//         mock::send_email::mock_implementation(|_to, _subject| {
//             Ok(())
//         });
//
//         // Call with specific user ID
//         let _ = mock::notify_user(42, "Test Subject".to_string());
//
//         // Verify the exact parameters
//         mock::fetch_user::assert_with(42);
//         mock::send_email::assert_with(("user_42@test.com".to_string(), "Test Subject".to_string()));
//
//         // Clean up
//         mock::fetch_user::clear_mock();
//         mock::send_email::clear_mock();
//     }
// }
