use std::fmt::Display;

#[fnmock::fakeable]
fn format_value(value: Box<dyn Display>) -> String {
    format!("Real {}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value_no_fake_for_dyn() {
        let result = format_value(Box::new(42));

        assert_eq!(result, "Real 42");
    }

    #[test]
    fn test_format_value_with_fake_for_dyn() {
        format_value_fake().setup(|value| format!("Fake {}", value));

        let result = format_value(Box::new(42));

        assert_eq!(result, "Fake 42");
    }
}
