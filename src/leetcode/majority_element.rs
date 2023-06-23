fn elem_count(nums: &Vec<i32>, left: i32, right: i32, v: i32) -> i32 {
    let mut cnt = 0;
    for n in left..=right {
        if nums.iter().nth(n as usize).unwrap().clone() == v {
            cnt += 1;
        }
    }
    cnt
}

fn helper(nums: &Vec<i32>, left: i32, right: i32) -> i32 {
    if left == right {
        return nums.iter().nth(left as usize).unwrap().clone();
    }

    let middle = left + (right - left) / 2;

    let left_majority = helper(nums, left, middle);
    let right_majority = helper(nums, middle + 1, right);

    println!("left majority = {}", left_majority);
    println!("right majority = {}", right_majority);

    if left_majority == right_majority {
        return left_majority;
    }

    let left_count = elem_count(nums, left, right, left_majority);
    let right_count = elem_count(nums, left, right, right_majority);

    println!("left count = {}", left_count);
    println!("right count = {}", right_count);

    if left_count > right_count {
        left_majority
    } else {
        right_majority
    }
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    helper(&nums, 0, (nums.len() - 1) as i32)
}
