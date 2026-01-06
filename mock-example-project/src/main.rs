use crate::generate_string::generate_string_from_digit;

mod generate_string;

fn generate_output(digit: u8) -> Result<String, String> {
    Ok(format!("Output: {}", generate_string_from_digit(digit)?))
}

fn main() {
    println!("{}", generate_output(2).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::generate_output;

    #[test]
    fn it_works() {
        let res = generate_output(2);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Output: Two");
    }

    #[test]
    fn it_propagates_the_error() {
        let res = generate_output(10);

        assert_eq!(res, Err("Digit should be between 0 and 9".to_string()));
    }
}