pub fn find_missing(seq: &[i32]) -> i32 {
    let n: i32 = seq.len() as i32;
    let a1 = seq.first().unwrap();
    let an = seq.last().unwrap();

    let dt: i32 = an - a1;

    if dt == 0 {
        *a1 as i32
    } else {
        let d: i32 = dt / n;

        let mut next_num = a1.clone();

        let mut result = 0;

        for i in seq {
            if *i == next_num {
                next_num = i + d;
            } else {
                result = i - d;
                break;
            }
        }

        result
    }
}
