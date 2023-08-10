pub fn reverse_words(s: String) -> String {
    let mut words: Vec<String> = vec![];

    for word in s.split_whitespace() {
        words.push(String::from(word));
    }

    (words.into_iter().rev().collect::<Vec<String>>()).join(" ")
}
