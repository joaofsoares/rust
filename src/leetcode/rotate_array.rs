pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() > 1 {
        for _ in 0..k {
            nums.rotate_right(1);
        }
    }
}
