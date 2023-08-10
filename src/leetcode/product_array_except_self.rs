pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut idx = 0;
    let end = nums.len();

    while idx < end {
        let left = &nums[0..idx];
        let right = &nums[(idx + 1)..end];

        let left_product: i32 = left.iter().product();
        let right_product: i32 = right.iter().product();

        res.push(left_product * right_product);

        idx += 1;
    }

    res
}
