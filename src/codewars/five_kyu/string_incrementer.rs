pub fn increment_string(s: &str) -> String {
    let rev_str: String = s.chars().rev().collect();

    let mut start_str: usize = 0;

    let mut rev_ns: Vec<char> = Vec::new();

    for i in 0..rev_str.len() {
        let ch = rev_str.chars().nth(i).unwrap();

        if ch.is_alphabetic() {
            start_str = s.len() - i;
            break;
        }

        if ch.is_numeric() {
            rev_ns.push(ch);
        }
    }

    let new_str = &s[0..start_str];

    let one = String::from("1");

    if rev_ns.is_empty() {
        return format!("{}{}", new_str, one);
    }

    rev_ns.reverse();

    let sum_str = add_str(rev_ns, one);

    format!("{}{}", new_str, sum_str)
}

fn add_str(x: Vec<char>, y: String) -> String {
    let mut tmp_x: Vec<char> = x.to_vec();
    let mut tmp_y: Vec<char> = y.chars().collect();
    let mut add_string = String::new();
    let mut add = 0;

    for _ in 0..tmp_x.len() {
        let a: u8 = match tmp_x.pop() {
            Some(c) => c.to_string().parse().unwrap(),
            _ => 0,
        };

        let b: u8 = match tmp_y.pop() {
            Some(c) => c.to_string().parse().unwrap(),
            _ => 0,
        };

        let mut sum: String = (a + b + add).to_string().chars().rev().collect();

        add = if sum.len() > 1 {
            sum.pop().unwrap().to_string().parse().unwrap()
        } else {
            0
        };

        add_string.push(*sum.chars().collect::<Vec<char>>().first().unwrap());
    }

    if add > 0 {
        add_string.push(
            *add.to_string()
                .chars()
                .collect::<Vec<char>>()
                .first()
                .unwrap(),
        );
    }

    add_string.chars().rev().collect::<String>()
}
