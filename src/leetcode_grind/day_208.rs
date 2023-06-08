// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/description/
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid[0].len();
    let mut count = 0;
    let mut row_neg_idx = n as i32 - 1;

    for row in grid {
        while row_neg_idx >= 0 && row[row_neg_idx as usize] < 0 {
            row_neg_idx -= 1;
        }
        count += (n as i32 - row_neg_idx - 1) as i32;
    }

    count
}
