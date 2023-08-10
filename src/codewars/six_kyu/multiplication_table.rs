pub fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    (1..=len)
        .map(|x| (1..=len).map(|y| x * y).collect())
        .collect()
}
