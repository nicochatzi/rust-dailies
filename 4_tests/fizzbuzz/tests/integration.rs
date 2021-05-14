// integration test is "outside" of our library.
// so it needs to "import"/bring into scope
// all of the types and methods we will use.
use fizzbuzz::*;

// there is no #[cfg(test)] here because this entire file is part
// only compiled when running tests. that's because this file is
// in the top level `test[s]/` directory.
#[test]
fn fizz_buzz_is_valid() {
    assert_eq!(
        fizz_buzz(0),
        FizzBuzzValue::String(String::from("fizzbuzz"))
    );
    assert_eq!(fizz_buzz(1), FizzBuzzValue::Number(1));
    assert_ne!(fizz_buzz(2), FizzBuzzValue::String(String::from("fizz")));
    assert_eq!(fizz_buzz(3), FizzBuzzValue::String(String::from("fizz")));
    assert_eq!(fizz_buzz(5), FizzBuzzValue::String(String::from("buzz")));
    assert_eq!(
        fizz_buzz(15),
        FizzBuzzValue::String(String::from("fizzbuzz"))
    );
}
