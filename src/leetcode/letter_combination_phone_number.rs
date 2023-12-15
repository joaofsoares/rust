use std::collections::VecDeque;

use crate::leetcode::solution::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let keyboard: Vec<String> = vec![
            String::from("0"),
            String::from("1"),
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
            String::from("jkl"),
            String::from("mno"),
            String::from("pqrs"),
            String::from("tuv"),
            String::from("wxyz"),
        ];

        let mut combs: Vec<String> = vec![];
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(String::new());

        while !queue.is_empty() {
            let s = &queue.pop_front().unwrap();

            if s.len() == digits.len() {
                combs.push(s.to_string());
            } else {
                let idx = digits.chars().nth(s.clone().len()).unwrap() as i32 - 0x30;
                let letters = keyboard[idx as usize].clone();
                for letter in letters.chars() {
                    let mut new_str = s.clone();
                    new_str.push(letter);
                    queue.push_back(new_str);
                }
            }
        }

        return combs;
    }
}

#[cfg(test)]
mod letter_combinations_tests {
    use super::*;

    #[test]
    fn test_combinations_one() {
        let digits = "23".to_string();
        let res = Solution::letter_combinations(digits);
        let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        assert_eq!(expected, res);
    }

    #[test]
    fn test_combinations_two() {
        let digits = "".to_string();
        let res = Solution::letter_combinations(digits);
        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, res);
    }

    #[test]
    fn test_combinations_three() {
        let digits = "2".to_string();
        let res = Solution::letter_combinations(digits);
        let expected = vec!["a", "b", "c"];
        assert_eq!(expected, res);
    }
}
