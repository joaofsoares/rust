pub fn narcissistic(num: u64) -> bool {
    let str_num = num.to_string();
    let size = str_num.len();

    let num_sum: u64 = str_num
        .chars()
        .map(|x| {
            let n = x.to_digit(10).unwrap() as u64;
            n.pow(size as u32)
        })
        .sum();

    num == num_sum
}
