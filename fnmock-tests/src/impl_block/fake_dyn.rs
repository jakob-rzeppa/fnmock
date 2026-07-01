use std::fmt::Display;

pub struct Formatter;

#[fnmock::fakeable]
impl Formatter {
    pub fn format_value(&self, value: Box<dyn Display>) -> String {
        format!("Real {}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value_no_fake_for_dyn() {
        let formatter = Formatter;
        let result = formatter.format_value(Box::new(42));

        assert_eq!(result, "Real 42");
    }

    #[test]
    fn test_format_value_with_fake_for_dyn() {
        Formatter::format_value_fake().setup(|_, value| format!("Fake {}", value));

        let formatter = Formatter;
        let result = formatter.format_value(Box::new(42));

        assert_eq!(result, "Fake 42");
    }
}
