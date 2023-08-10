pub fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut words: Vec<String> = s.split_whitespace().map(str::to_string).collect();
    words.sort_by_key(|x| x.chars().last());
    words
}
