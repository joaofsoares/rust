use crate::leetcode::solution::Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut formatted_str = s.trim().to_string();

        if formatted_str.len() == 0 {
            return 0;
        }

        let is_neg = if formatted_str.chars().nth(0).unwrap() == '-' {
            formatted_str.remove(0);
            -1
        } else if formatted_str.chars().nth(0).unwrap() == '+' {
            formatted_str.remove(0);
            1
        } else {
            1
        };

        let mut digits: i64 = 0;

        for i in 0..formatted_str.len() {
            if !formatted_str.chars().nth(i).unwrap().is_digit(10) {
                break;
            }

            digits =
                digits * 10 + formatted_str.chars().nth(i).unwrap().to_digit(10).unwrap() as i64;

            if is_neg * digits < std::i32::MIN as i64 {
                return std::i32::MIN;
            } else if is_neg * digits > std::i32::MAX as i64 {
                return std::i32::MAX;
            }
        }

        return (is_neg * digits) as i32;
    }
}

#[cfg(test)]
mod string_to_integer_tests {
    use super::*;

    #[test]
    fn test_conversion_one() {
        let s = String::from("42");
        let res = Solution::my_atoi(s);
        assert_eq!(res, 42);
    }

    #[test]
    fn test_conversion_two() {
        let s = String::from("     -42");
        let res = Solution::my_atoi(s);
        assert_eq!(res, -42);
    }

    #[test]
    fn test_conversion_three() {
        let s = String::from("4193 with words");
        let res = Solution::my_atoi(s);
        assert_eq!(res, 4193);
    }

    #[test]
    fn test_conversion_four() {
        let s = String::from("4193 with other number 32");
        let res = Solution::my_atoi(s);
        assert_eq!(res, 4193);
    }

    #[test]
    fn test_conversion_five() {
        let s = String::from("00004193 with other number 32");
        let res = Solution::my_atoi(s);
        assert_eq!(res, 4193);
    }

    #[test]
    fn test_conversion_six() {
        let s = String::from("words and 987");
        let res = Solution::my_atoi(s);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_conversion_seven() {
        let s = String::from("+1");
        let res = Solution::my_atoi(s);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_conversion_eight() {
        let s = String::from("00000-42a1234");
        let res = Solution::my_atoi(s);
        assert_eq!(res, 0);
    }
}
