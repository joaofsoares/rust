fn sum_strings(num1: String, num2: String) -> String {
    let mut n1_len: i32 = num1.len() as i32 - 1;
    let mut n2_len: i32 = num2.len() as i32 - 1;

    let mut carry = '0';

    let mut res = String::from("");

    while n1_len >= 0 && n2_len >= 0 {
        let s = (num1
            .chars()
            .nth(n1_len as usize)
            .unwrap()
            .to_digit(10)
            .unwrap()
            + num2
                .chars()
                .nth(n2_len as usize)
                .unwrap()
                .to_digit(10)
                .unwrap()
            + carry.to_digit(10).unwrap())
        .to_string();

        if s.len() > 1 {
            carry = s.chars().nth(0).unwrap();
            res.push(s.chars().last().unwrap());
        } else {
            res.push(s.chars().nth(0).unwrap());
            carry = '0';
        }

        n1_len -= 1;
        n2_len -= 1;
    }

    while n1_len >= 0 {
        let s = (num1
            .chars()
            .nth(n1_len as usize)
            .unwrap()
            .to_digit(10)
            .unwrap()
            + carry.to_digit(10).unwrap())
        .to_string();

        if s.len() > 1 {
            carry = s.chars().nth(0).unwrap();
            res.push(s.chars().last().unwrap());
        } else {
            res.push(s.chars().nth(0).unwrap());
            carry = '0';
        }

        n1_len -= 1;
    }

    while n2_len >= 0 {
        let s = (num2
            .chars()
            .nth(n2_len as usize)
            .unwrap()
            .to_digit(10)
            .unwrap()
            + carry.to_digit(10).unwrap())
        .to_string();

        if s.len() > 1 {
            carry = s.chars().nth(0).unwrap();
            res.push(s.chars().last().unwrap());
        } else {
            res.push(s.chars().nth(0).unwrap());
            carry = '0';
        }

        n2_len -= 1;
    }

    if carry != '0' {
        res.push(carry);
    }

    res.chars().rev().collect()
}

pub fn multiply(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" {
        return String::from("0");
    }

    let mut sums: Vec<String> = vec![];

    for n1 in num1.chars().rev() {
        let mut sum = String::from("");
        let mut carry = '0';

        for n2 in num2.chars().rev() {
            let t = (n1.to_digit(10).unwrap() * n2.to_digit(10).unwrap()
                + carry.to_digit(10).unwrap())
            .to_string();

            if t.len() > 1 {
                carry = t.chars().nth(0).unwrap();
                sum.push(t.chars().last().unwrap());
            } else {
                sum.push(t.chars().nth(0).unwrap());
                carry = '0';
            }
        }

        if carry != '0' {
            sum.push(carry);
        }

        sums.push(sum.chars().rev().collect());
    }

    let mut total = String::from("0");
    let mut cnt = 0;
    for s in sums {
        total = sum_strings(format!("{}{}", s, "0".repeat(cnt)), total);
        cnt += 1;
    }

    total
}
