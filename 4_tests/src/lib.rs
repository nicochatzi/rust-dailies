// Debug is required when panicking.
// panic!() will try to log to console
// an error message which will contain
// FizzBuzzValue. These derives mean
// the compiler will supply default
// implementations of the functionality.
// Equality is to allow comparing 2
// FizzBuzzValue instances.
#[derive(Debug, PartialEq)]
pub enum FizzBuzzValue {
    Number(u32),
    String(String),
}

pub fn fizz_buzz(n: u32) -> FizzBuzzValue {
    match get_fizz_buzz_string(n) {
        Some(string) => FizzBuzzValue::String(string),
        None => FizzBuzzValue::Number(n),
    }
}

fn get_fizz_buzz_string(n: u32) -> Option<String> {
    let mut string = String::new();

    if is_divisible(n, 3) {
        string.push_str("fizz");
    }

    if is_divisible(n, 5) {
        string.push_str("buzz");
    }

    if string.is_empty() {
        None
    } else {
        Some(string)
    }
}

fn is_divisible(x: u32, y: u32) -> bool {
    x % y == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // run just this test with: cargo test can_get_fizz_buzz_strings
    // run test and see stdout: cargo test -- --nocapture
    #[test]
    fn numbers_are_divisible() {
        assert!(is_divisible(4, 2));
        assert!(!is_divisible(5, 3));

        println!(
            "x: {}, y: {}, are divisible ? {}",
            10,
            5,
            is_divisible(10, 5)
        );
    }

    // run all unit tests within the library: cargo test --lib
    #[test]
    fn can_get_fizz_buzz_strings() {
        assert_eq!(get_fizz_buzz_string(1), None);
        assert_eq!(get_fizz_buzz_string(2), None);
        assert_ne!(get_fizz_buzz_string(2), Some(String::from("fizz")));
        assert_eq!(get_fizz_buzz_string(3), Some(String::from("fizz")));
        assert_eq!(get_fizz_buzz_string(5), Some(String::from("buzz")));
        assert_eq!(get_fizz_buzz_string(15), Some(String::from("fizzbuzz")));
    }
}
