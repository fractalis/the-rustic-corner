pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = num.to_string().len() as u32;

    let num_str = num.to_string();
    let res = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_digits) as u64)
        .sum::<u64>();

    num as u64 == res
}
