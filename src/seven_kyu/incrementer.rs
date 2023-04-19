fn get_new_value(v: u32) -> u32 {
    let s = v.to_string();
    let l = s.chars().last().unwrap();
    l.to_digit(10).unwrap()
}

pub fn incrementer(nums: &[u32]) -> Vec<u32> {
    let mut cnt = 1;
    let mut rst: Vec<u32> = Vec::new();

    for i in nums {
        let v = i + cnt;

        let nv = if v > 9 { get_new_value(v) } else { v };

        rst.push(nv);
        cnt += 1;
    }

    rst
}
