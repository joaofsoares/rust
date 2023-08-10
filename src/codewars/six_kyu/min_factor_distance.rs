pub fn min_distance(n: u32) -> u32 {
    let mut nums: Vec<u32> = vec![];

    for i in 1..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            if n / i != i {
                nums.push(i);
                nums.push(n / i);
            }
        }
    }

    nums.sort_by(|a, b| b.cmp(a));

    return 0;
}
