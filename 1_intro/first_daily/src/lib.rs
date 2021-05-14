pub fn is_even(n: u32) -> bool {
    n % 2 == 0
}

#[test]
fn even_numbers_are_even() {
    assert!(is_even(2));
    assert!(is_even(4));
    assert!(is_even(6));
}