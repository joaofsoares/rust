use crate::leetcode::solution::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            if !Self::validate_line(&board, i) || !Self::validate_column(&board, i) {
                return false;
            }
        }

        Self::validate_blocks(&board)
    }

    fn validate_line(board: &Vec<Vec<char>>, row: i32) -> bool {
        let mut passed = Vec::new();

        for v in board[row as usize].iter() {
            if v == &'.' {
                continue;
            } else if passed.contains(&v) {
                return false;
            } else {
                passed.push(v);
            }
        }

        true
    }

    fn validate_column(board: &Vec<Vec<char>>, col: i32) -> bool {
        let mut passed = Vec::new();

        for row in (0..9).into_iter() {
            if board[row][col as usize] == '.' {
                continue;
            } else if passed.contains(&board[row][col as usize]) {
                return false;
            } else {
                passed.push(board[row][col as usize]);
            }
        }

        true
    }

    fn validate_blocks(board: &Vec<Vec<char>>) -> bool {
        for row in (0..9).step_by(3) {
            for col in (0..9).step_by(3) {
                let mut passed = Vec::new();
                for r in row..(row + 3) {
                    for c in col..(col + 3) {
                        if board[r][c] == '.' {
                            continue;
                        } else if passed.contains(&board[r][c]) {
                            return false;
                        } else {
                            passed.push(board[r][c]);
                        }
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod valid_sudoku_test {

    use crate::leetcode::valid_sudoku::Solution;

    #[test]
    pub fn test_valid_sudoku_one() {
        let input = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let res = Solution::is_valid_sudoku(input);

        assert_eq!(res, true);
    }

    #[test]
    pub fn test_valid_sudoku_two() {
        let input = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let res = Solution::is_valid_sudoku(input);

        assert_eq!(res, false);
    }
}
