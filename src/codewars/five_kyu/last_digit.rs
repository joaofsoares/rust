pub fn last_digit(str1: &str, str2: &str) -> i32 {
    let len_a = str1.len();
    let len_b = str2.len();

    if len_a == 1 && len_b == 1 && get_nth_char(str2, 0) == '0' && get_nth_char(str1, 0) == '0' {
        return 1;
    }

    if len_b == 1 && get_nth_char(str2, 0) == '0' {
        return 1;
    }

    if len_a == 1 && get_nth_char(str1, 0) == '0' {
        return 0;
    }

    let res_mod: i32 = get_mod_four(4, str2);

    let exp: i32 = if res_mod == 0 { 4 } else { res_mod };

    let base: i32 = convert_char_to_digit(get_last_char(str1));

    let temp = base.pow(exp as u32);

    convert_char_to_digit(get_last_char(&temp.to_string()))
}

fn get_mod_four(a: i32, b: &str) -> i32 {
    let mut m: i32 = 0;
    for i in 0..b.len() {
        m = (m * 10 + convert_char_to_digit(get_nth_char(b, i))) % a;
    }
    m
}

fn convert_char_to_digit(c: char) -> i32 {
    c.to_digit(10).unwrap() as i32
}

fn get_nth_char(s: &str, idx: usize) -> char {
    s.chars().nth(idx).unwrap()
}

fn get_last_char(s: &str) -> char {
    s.chars().last().unwrap()
}
