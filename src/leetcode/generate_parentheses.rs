use crate::leetcode::solution::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut parenthesis: Vec<String> = vec![];

        Self::combinations(&mut parenthesis, "".to_string(), 0, 0, n);

        parenthesis
    }

    fn combinations(pars: &mut Vec<String>, s: String, open: i32, close: i32, n: i32) {
        if open == n && close == n {
            pars.push(s.to_string());
            return;
        }

        if open < n {
            Self::combinations(pars, s.to_string() + "(", open + 1, close, n);
        }

        if close < open {
            Self::combinations(pars, s.to_string() + ")", open, close + 1, n);
        }
    }
}

#[cfg(test)]
mod generate_parenthesis_test {
    use crate::leetcode::solution::Solution;

    #[test]
    fn generate_parenthesis_one() {
        let result = Solution::generate_parenthesis(3);

        let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];

        assert_eq!(expected, result);
    }

    #[test]
    fn generate_parenthesis_two() {
        let result = Solution::generate_parenthesis(1);

        let expected = vec!["()"];

        assert_eq!(expected, result);
    }
}
