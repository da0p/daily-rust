/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cc_iter = code
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
