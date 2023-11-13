// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let cc_iter = cc_number
        .chars()
        .filter(|character| !character.is_whitespace())
        .map(|character| character.to_digit(10))
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| {
            if i % 2 == 1 && digit.is_some() {
                let doubled = digit.unwrap() * 2;
                if doubled > 9 {
                    Some(doubled / 10 + doubled % 10)
                } else {
                    Some(doubled)
                }
            } else {
                digit
            }
        })
        .collect::<Vec<Option<u32>>>();

    if cc_iter.len() < 2 {
        return false;
    }

    let mut sum: u32 = 0;
    for digit in cc_iter.into_iter() {
        if digit.is_none() {
            return false;
        }
        sum += digit.unwrap();
    }

    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
