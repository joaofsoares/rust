use std::collections::HashMap;

pub fn valid_braces(s: &str) -> bool {
    let mut tmp: Vec<char> = Vec::new();

    let opposite = HashMap::from([(')', '('), (']', '['), ('}', '{')]);

    let opening = vec!['(', '{', '['];

    for c in s.chars().into_iter() {
        if opening.contains(&c) {
            tmp.push(c);
        } else {
            let l = match tmp.pop() {
                Some(v) => v,
                _ => '~',
            };

            let opp = match opposite.get(&c) {
                Some(v) => v,
                _ => &'-',
            };

            if &l != opp {
                return false;
            }
        }
    }

    tmp.is_empty()
}
