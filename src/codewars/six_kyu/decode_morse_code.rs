use std::collections::HashMap;

pub fn decode(s: &str) -> String {
    let MORSE_CODES = create_codes_map();

    let word = clean_word(s.trim());

    let words = word.split(" ");

    words
        .map(|w| MORSE_CODES.get(w).unwrap().to_string())
        .into_iter()
        .collect::<String>()
}

fn clean_word(s: &str) -> String {
    s.replace("   ", " | ")
}

fn create_codes_map() -> HashMap<String, String> {
    HashMap::from([
        (String::from(".-"), String::from("A")),
        (String::from("-..."), String::from("B")),
        (String::from("-.-."), String::from("C")),
        (String::from("-.."), String::from("D")),
        (String::from("."), String::from("E")),
        (String::from("..-."), String::from("F")),
        (String::from("--."), String::from("G")),
        (String::from("...."), String::from("H")),
        (String::from(".."), String::from("I")),
        (String::from(".---"), String::from("J")),
        (String::from("-.-"), String::from("K")),
        (String::from(".-.."), String::from("L")),
        (String::from("--"), String::from("M")),
        (String::from("-."), String::from("N")),
        (String::from("---"), String::from("O")),
        (String::from(".--."), String::from("P")),
        (String::from("--.-"), String::from("Q")),
        (String::from(".-."), String::from("R")),
        (String::from("..."), String::from("S")),
        (String::from("-"), String::from("T")),
        (String::from("..-"), String::from("U")),
        (String::from("...-"), String::from("V")),
        (String::from(".--"), String::from("W")),
        (String::from("-..-"), String::from("X")),
        (String::from("-.--"), String::from("Y")),
        (String::from("--.."), String::from("Z")),
        (String::from(" "), String::from("")),
        (String::from("|"), String::from(" ")),
    ])
}
