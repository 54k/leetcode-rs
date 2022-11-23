#[allow(dead_code)]
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;
    let mut cache = HashSet::with_capacity(board.len() * board.len());
    for r in 0..board.len() {
        for c in 0..board.len() {
            let ch = board[r][c];
            if ch != '.'
                && (!cache.insert(format!("{} in row {}", ch, r))
                    || !cache.insert(format!("{} in col {}", ch, c))
                    || !cache.insert(format!("{} in sub-square {} {}", ch, r / 3, c / 3)))
            {
                return false;
            }
        }
    }
    true
}

#[allow(dead_code)]
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();
    for i in 0..(len + 1) / 2 {
        for j in 0..len / 2 {
            let temp = matrix[len - 1 - j][i];
            matrix[len - 1 - j][i] = matrix[len - 1 - i][len - j - 1];
            matrix[len - 1 - i][len - j - 1] = matrix[j][len - 1 - i];
            matrix[j][len - 1 - i] = matrix[i][j];
            matrix[i][j] = temp;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day_11::{is_valid_sudoku, rotate};

    #[test]
    fn test64() {
        println!(
            "{}",
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '7'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '.', '9'],
            ])
        );
    }

    #[test]
    fn test65() {
        let mut matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate(&mut matrix1);
        println!("{:?}", matrix1);
        let mut matrix2 = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut matrix2);
        println!("{:?}", matrix2);
    }
}
