pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let tmp = my_pow(x, n / 2);

    if n % 2 == 0 {
        tmp * tmp
    } else {
        if n > 0 {
            x * tmp * tmp
        } else {
            (tmp * tmp) / x
        }
    }
}
