pub fn reverse_letters(s: &str) -> String {
    s.chars()
        .rev()
        .into_iter()
        .filter(|c| c.is_alphabetic())
        .collect()
}

#[cfg(test)]
mod reverse_letters_tests {
    use super::*;

    #[test]
    fn reverse_letters_test() {
        assert_eq!("abc", reverse_letters("c2b3a4"));
    }
}
