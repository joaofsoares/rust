use std::collections::HashMap;

pub fn count(input: &str) -> HashMap<char, i32> {
    let mut m = HashMap::new();

    for c in input.chars() {
        let count = m.entry(c).or_insert(0);
        *count += 1;
    }

    m
}
