use crate::leetcode::solution::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let (mut low, mut high) = (0, (nums.len() as i32 - 1));

        while low <= high {
            let mid = (low + high) / 2;

            if target == nums[mid as usize] {
                return mid;
            } else if nums[mid as usize] < target {
                if nums[low as usize] > nums[mid as usize] && nums[low as usize] <= target {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                if nums[high as usize] < nums[mid as usize] && nums[high as usize] >= target {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod search_rotated_sorted_array_tests {
    use super::*;

    #[test]
    fn test_search_one() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let res = Solution::search(nums, 0);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_search_four() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2, 3];
        let res = Solution::search(nums, 0);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_search_two() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let res = Solution::search(nums, 3);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_search_three() {
        let nums = vec![1];
        let res = Solution::search(nums, 0);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_search_five() {
        let nums = vec![1, 3];
        let res = Solution::search(nums, 3);
        assert_eq!(res, 1);
    }
}
