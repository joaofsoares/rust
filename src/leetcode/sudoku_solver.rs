use crate::leetcode::solution::Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
        let mut row: usize = 0;
        let mut col: usize = 0;

        if !Self::find_unassigned_location(board, &mut row, &mut col) {
            return true;
        }

        for ch in '1'..='9' {
            if Self::is_valid(board, &mut row, &mut col, ch) {
                board[row][col] = ch;

                if Self::solve_sudoku(board) {
                    return true;
                }

                board[row][col] = '.';
            }
        }

        false
    }

    fn find_unassigned_location(board: &Vec<Vec<char>>, row: &mut usize, col: &mut usize) -> bool {
        for r in 0..9 {
            *row = r;
            for c in 0..9 {
                *col = c;
                if board[*row][*col] == '.' {
                    return true;
                }
            }
        }
        false
    }

    fn is_valid(board: &Vec<Vec<char>>, row: &mut usize, col: &mut usize, c: char) -> bool {
        !Self::find_in_row(board, row, c)
            && !Self::find_in_col(board, col, c)
            && !Self::find_in_square(board, *row - *row % 3, *col - *col % 3, c)
            && board[*row][*col] == '.'
    }

    fn find_in_row(board: &Vec<Vec<char>>, row: &mut usize, c: char) -> bool {
        for col in 0..9 {
            if board[*row][col] == c {
                return true;
            }
        }
        false
    }

    fn find_in_col(board: &Vec<Vec<char>>, col: &mut usize, c: char) -> bool {
        for row in 0..9 {
            if (board[row][*col]) == c {
                return true;
            }
        }
        false
    }

    fn find_in_square(board: &Vec<Vec<char>>, start_row: usize, start_col: usize, c: char) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if board[row + start_row][col + start_col] == c {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod sudoku_solver {
    use crate::leetcode::solution::Solution;

    #[test]
    pub fn test_sudoku_one() {
        let mut input = vec![
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

        Solution::solve_sudoku(&mut input);

        let expected = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        assert_eq!(expected, input);
    }
}
