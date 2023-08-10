pub fn camel_case(str: &str) -> String {
    str.split_whitespace()
        .map(str::to_string)
        .map(capitalize)
        .collect::<Vec<String>>()
        .join("")
}

fn capitalize(s: String) -> String {
    let mut cap = String::new();

    for (k, v) in s.chars().enumerate() {
        if k == 0 {
            cap.push_str(v.to_string().to_uppercase().as_str());
        } else {
            cap.push_str(v.to_string().to_lowercase().as_str());
        }
    }

    cap
}
