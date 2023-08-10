pub fn partitions(n: u32) -> u32 {
    let mut parts = vec![1];

    for _ in 0..n {
        parts.push(0);
    }

    for i in 1..n + 1 {
        for j in i..n + 1 {
            parts[j as usize] += parts[j as usize - i as usize];
        }
    }

    parts.into_iter().last().unwrap() as u32
}
