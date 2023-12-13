use std::cmp;
use std::collections::VecDeque;

use crate::leetcode::solution::Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut longest = 0;
        let mut pars = VecDeque::new();
        pars.push_back(-1 as i32);

        for i in 0..s.len() {
            if s.chars().nth(i).unwrap() == '(' {
                pars.push_back(i as i32);
            } else if pars.len() == 1 {
                pars[0] = i as i32;
            } else {
                pars.pop_back();
                longest = cmp::max(longest, i as i32 - pars.back().unwrap())
            }
        }

        return longest;
    }
}

#[cfg(test)]
mod longest_valid_parentheses_tests {
    use super::*;

    #[test]
    fn test_valid_parentheses_one() {
        let s = String::from("(()");
        let res = Solution::longest_valid_parentheses(s);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_valid_parentheses_two() {
        let s = String::from(")()())");
        let res = Solution::longest_valid_parentheses(s);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_valid_parentheses_three() {
        let s = String::from("");
        let res = Solution::longest_valid_parentheses(s);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_valid_parentheses_four() {
        let s = String::from("()(()");
        let res = Solution::longest_valid_parentheses(s);
        assert_eq!(res, 2);
    }
}
