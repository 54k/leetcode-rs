#[allow(dead_code)]
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;
    let mut cache = HashSet::with_capacity(board.len() * board[0].len());
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

#[cfg(test)]
mod test {
    use crate::day_11::is_valid_sudoku;

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
}
