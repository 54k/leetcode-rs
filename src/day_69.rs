use std::hash::Hash;
use std::io::Write;

// https://leetcode.com/problems/non-decreasing-subsequences/solutions/2910678/increasing-subsequences/?orderBy=most_relevant
pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::*;
    fn backtrack(nums: &[i32], i: usize, cur: &mut Vec<i32>, res: &mut HashSet<Vec<i32>>) {
        if i >= nums.len() {
            if cur.len() >= 2 {
                res.insert(cur.clone());
            }
            return;
        }
        if cur.is_empty() || *cur.last().unwrap() <= nums[i] {
            cur.push(nums[i]);
            backtrack(nums, i + 1, cur, res);
            cur.pop();
        }
        backtrack(nums, i + 1, cur, res);
    }
    let mut res = HashSet::new();
    backtrack(&nums, 0, &mut vec![], &mut res);
    res.into_iter().collect()
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    use std::collections::*;
    fn backtrack(
        board: &mut Vec<Vec<char>>,
        mut i: usize,
        mut j: usize,
        solved: &mut bool,
        cache: &mut HashSet<String>,
    ) {
        let mut is_full = true;
        for r in 0..board.len() {
            for c in 0..board.len() {
                if board[r][c] == '.' {
                    is_full = false;
                }
            }
        }
        if is_full {
            *solved = true;
            return;
        }

        while board[i][j] != '.' {
            if j < board.len() - 1 {
                j += 1;
            } else {
                i += 1;
                j = 0;
            }
        }

        for d in 1..=9 {
            let ch = char::from_digit(d as u32, 10).unwrap();

            let row_key = format!("{} in row {}", ch, i);
            let col_key = format!("{} in col {}", ch, j);
            let sub_square_key = format!("{} in sub-square {} {}", ch, i / 3, j / 3);

            if cache.contains(&row_key)
                || cache.contains(&col_key)
                || cache.contains(&sub_square_key)
            {
                continue;
            }

            cache.insert(row_key.clone());
            cache.insert(col_key.clone());
            cache.insert(sub_square_key.clone());

            board[i][j] = ch;
            backtrack(board, i, j, solved, cache);

            if !*solved {
                cache.remove(&row_key);
                cache.remove(&col_key);
                cache.remove(&sub_square_key);
                board[i][j] = '.';
            }
        }
    }

    let mut cache = HashSet::new();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let ch = board[i][j];
            if ch != '.' {
                cache.insert(format!("{} in row {}", ch, i));
                cache.insert(format!("{} in col {}", ch, j));
                cache.insert(format!("{} in sub-square {} {}", ch, i / 3, j / 3));
            }
        }
    }
    backtrack(board, 0, 0, &mut false, &mut cache);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test162() {
        println!("{:?}", find_subsequences(vec![4, 6, 7, 7])); // [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
    }

    #[test]
    fn test163() {
        let mut board = vec![
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
        solve_sudoku(&mut board);
        println!("BOARD1");
        board.into_iter().for_each(|r| {
            println!("{:?}", r);
        });

        let mut board2 = vec![
            vec!['.', '.', '9', '7', '4', '8', '.', '.', '.'],
            vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '2', '.', '1', '.', '9', '.', '.', '.'],
            vec!['.', '.', '7', '.', '.', '.', '2', '4', '.'],
            vec!['.', '6', '4', '.', '1', '.', '5', '9', '.'],
            vec!['.', '9', '8', '.', '.', '.', '3', '.', '.'],
            vec!['.', '.', '.', '8', '.', '3', '.', '2', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '6'],
            vec!['.', '.', '.', '2', '7', '5', '9', '.', '.'],
        ];
        solve_sudoku(&mut board2);
        println!("BOARD2");
        board2.into_iter().for_each(|r| {
            println!("{:?}", r);
        });
    }
}
