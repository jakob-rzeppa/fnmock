mod generate;

use fnmock::derive::use_function_mock;

#[use_function_mock]
use generate::generate_string_from_digit;

fn generate_output(digit: u8) -> Result<String, String> {
    Ok(format!("Output: {}", generate_string_from_digit(digit)?))
}

#[cfg(test)]
mod tests {
    use crate::generate_string::generate::generate_string_from_digit_mock;
    use crate::generate_string::generate_output;

    #[test]
    fn it_works() {
        generate_string_from_digit_mock::mock_implementation(|_: u8| {
            Ok("Mock Output".to_string())
        });

        let res = generate_output(2);

        generate_string_from_digit_mock::assert_times(1);
        generate_string_from_digit_mock::assert_with(2);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Output: Mock Output");
    }

    #[test]
    fn it_propagates_the_error() {
        generate_string_from_digit_mock::mock_implementation(|_: u8| {
            Err("Mock Error".to_string())
        });

        let res = generate_output(2);

        generate_string_from_digit_mock::assert_times(1);
        generate_string_from_digit_mock::assert_with(2);
        assert_eq!(res, Err("Mock Error".to_string()));
    }
}