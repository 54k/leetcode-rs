// https://leetcode.com/problems/surrounded-regions/description
pub fn solve(board: &mut Vec<Vec<char>>) {
    struct Pair {
        first: usize,
        second: usize,
    }

    fn dfs(board: &mut Vec<Vec<char>>, row: i32, col: i32) {
        if board[row as usize][col as usize] != 'O' {
            return;
        }

        let ROWS = board.len() as i32;
        let COLS = board[0].len() as i32;

        board[row as usize][col as usize] = 'E';
        if col < COLS - 1 {
            dfs(board, row, col + 1);
        }
        if row < ROWS - 1 {
            dfs(board, row + 1, col);
        }
        if col > 0 {
            dfs(board, row, col - 1);
        }
        if row > 0 {
            dfs(board, row - 1, col);
        }
    }

    if board.len() == 0 {
        return;
    }

    let ROWS = board.len();
    let COLS = board[0].len();

    let mut borders = vec![];
    for r in 0..ROWS {
        borders.push(Pair {
            first: r,
            second: 0,
        });
        borders.push(Pair {
            first: r,
            second: COLS - 1,
        });
    }
    for c in 0..COLS {
        borders.push(Pair {
            first: 0,
            second: c,
        });
        borders.push(Pair {
            first: ROWS - 1,
            second: c,
        });
    }

    for pair in borders {
        dfs(board, pair.first as i32, pair.second as i32);
    }

    for r in 0..ROWS {
        for c in 0..COLS {
            if board[r][c] == 'O' {
                board[r][c] = 'X';
            }
            if board[r][c] == 'E' {
                board[r][c] = 'O';
            }
        }
    }
}
