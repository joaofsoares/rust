pub fn count_nines(n: u64) -> u64 {
    let len = n.to_string().len();

    let mut cnt: u64 = 0;

    for i in 0..=len {
        cnt += calc_digits(&n, &i);
    }

    cnt
}

fn calc_digits(n: &u64, i: &usize) -> u64 {
    let idx = i.clone();
    let d = idx as u32;

    let base: u64 = 10;

    let powed = base.pow(d);
    let double_powed = powed * 10;
    let dec = n % powed;

    let round_down = n - n % double_powed;
    let round_up = round_down + double_powed;

    let digit = (n / powed) % 10;

    if digit < 9 {
        round_down / 10
    } else if digit == 9 {
        round_down / 10 + dec + 1
    } else {
        round_up / 10
    }
}
