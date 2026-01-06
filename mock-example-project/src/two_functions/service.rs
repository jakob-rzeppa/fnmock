use mock_lib::derive::function_mock;

// Fetches user data from a database
#[function_mock]
pub fn fetch_user(id: u32) -> Result<String, String> {
    Ok(format!("User_{}", id))
}

#[function_mock]
pub fn send_email(user: String, body: String) -> Result<(), String> {
    println!("Send email to {}: {}", user, body);

    Ok(())
}