pub fn get_strings(s: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let mut fmt_strs: Vec<String> = Vec::new();

    fmt_strs.push(s);

    while !fmt_strs.is_empty() {
        let mut tmp_str = fmt_strs.pop().unwrap().clone();

        let idx = tmp_str.chars().position(|p| p == '?');

        if idx.is_some() {
            let n_idx = idx.unwrap();
            for i in 0..=1 {
                tmp_str.replace_range(n_idx..n_idx + 1, i.to_string().as_str());
                fmt_strs.push(tmp_str.to_string());
            }
        } else {
            res.push(tmp_str.clone());
        }
    }

    res
}
