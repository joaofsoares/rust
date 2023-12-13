use crate::leetcode::solution::Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut x = 1;
        nums.sort();

        for &n in nums.iter() {
            if n == x {
                x += 1;
            }
        }

        return x;
    }
}

#[cfg(test)]
mod first_missing_positive_tests {
    use super::*;

    #[test]
    fn test_missing_positive_one() {
        let nums = vec![1, 2, 0];
        let res = Solution::first_missing_positive(nums);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_missing_positive_two() {
        let nums = vec![3, 4, -1, 1];
        let res = Solution::first_missing_positive(nums);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_missing_positive_three() {
        let nums = vec![7, 8, 9, 11, 12];
        let res = Solution::first_missing_positive(nums);
        assert_eq!(res, 1);
    }
}
