pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut s_cits = citations.clone();
    s_cits.sort_by(|a, b| b.cmp(a));

    let mut cnt = 0;
    let mut res = 0;
    let mut size: i32 = (citations.len() - 1) as i32;

    while cnt <= size {
        let mid = (cnt + size) / 2;

        //if s_cits.iter().nth(mid as usize).unwrap() >= &((mid + 1) as i32) {
        if &s_cits[mid as usize] >= &((mid + 1) as i32) {
            cnt = mid + 1;

            res = mid + 1;
        } else {
            size = mid - 1;
        }
    }

    res as i32
}
