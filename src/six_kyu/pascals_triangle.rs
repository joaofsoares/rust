pub fn pascals_triangle(n: usize) -> Vec<usize> {
    let mut rows: Vec<usize> = Vec::new();

    for line in 1..=n {
        let mut d = 1;
        rows.push(d);
        for i in 1..line {
            d = d * (line - i) / i;
            rows.push(d);
        }
    }

    rows
}
