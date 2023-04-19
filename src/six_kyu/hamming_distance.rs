pub fn hamming(a: &str, b: &str) -> usize {
    (0..a.len())
        .filter(|i| a.chars().nth(*i).unwrap() != b.chars().nth(*i).unwrap())
        .count()
}

pub fn another_hamming(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}
