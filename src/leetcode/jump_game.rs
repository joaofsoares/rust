pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }

    let mut max = nums.iter().nth(0).unwrap().clone();

    if max == 0 {
        return false;
    }

    for i in 1..nums.len() {
        let curr = nums.iter().nth(i).unwrap();

        if max >= nums.len() as i32 - 1 {
            return true;
        }

        if max <= (i as i32) && curr.clone() == 0 {
            return false;
        }

        if (i as i32) + curr > max {
            max = (i as i32) + curr;
        }
    }

    false
}
