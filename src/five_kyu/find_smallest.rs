pub fn smallest(n: i64) -> (i64, usize, usize) {
    let s = n.to_string();

    let mut m: i64 = n;
    let mut f: usize = 0;
    let mut t: usize = 0;

    for i in 0..s.len() {
        let r: String = s[..i].to_owned() + &s[i + 1..];

        for j in 0..r.len() + 1 {
            let num_str =
                r[..j].to_string() + s.chars().nth(i).unwrap().to_string().as_str() + &r[j..];

            let num = num_str.parse::<i64>().unwrap();

            if num < m {
                m = num;
                f = i;
                t = j;
            }
        }
    }

    (m, f, t)
}
