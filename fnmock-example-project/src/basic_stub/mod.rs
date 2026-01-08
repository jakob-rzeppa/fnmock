use fnmock::derive::{use_function_stub, use_stub_inline};

mod config {
    use fnmock::derive::stub_function;

    #[stub_function]
    pub fn get_config() -> String {
        // Real implementation
        "production_config".to_string()
    }

    #[stub_function]
    pub fn get_port() -> u16 {
        8080
    }
}

#[use_function_stub]
use config::get_config;

fn process_config() -> String {
    get_config()
}

fn process_port() -> u16 {
    use_stub_inline!(config::get_port)()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::config::{get_config_stub, get_port_stub};

    #[test]
    fn test_stub_with_use_function_stub() {
        // Set up stub
        get_config_stub::setup("test_config".to_string());

        // Call the function that uses the stub
        let result = process_config();

        // Verify result
        assert_eq!(result, "test_config");

        // Clean up
        get_config_stub::clear();
    }

    #[test]
    fn test_stub_with_use_stub_inline() {
        // Set up stub
        get_port_stub::setup(3000);

        // Call the function that uses the stub inline
        let result = process_port();

        // Verify result
        assert_eq!(result, 3000);

        // Clean up
        get_port_stub::clear();
    }
}
