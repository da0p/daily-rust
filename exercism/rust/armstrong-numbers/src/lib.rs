pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = vec![];
    let mut initial_num = num;
    loop {
        digits.push((initial_num % 10) as u64);
        initial_num = initial_num / 10;
        if initial_num == 0 {
            break;
        }
    }

    let sum: u64 = digits.iter().map(|d| {
        d.pow(digits.len() as u32)
    }).sum();

    sum == num.into()
}
