struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in nums {
            res ^= n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::single_number::Solution;

    #[test]
    fn test_single_number_1() {
        let nums = vec![2, 2, 1];

        let res = Solution::single_number(nums);

        assert_eq!(res, 1);
    }

    #[test]
    fn test_single_number_2() {
        let nums = vec![4, 1, 2, 1, 2];

        let res = Solution::single_number(nums);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_single_number_3() {
        let nums = vec![1];

        let res = Solution::single_number(nums);

        assert_eq!(res, 1);
    }
}
