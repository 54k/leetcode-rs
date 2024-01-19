// https://leetcode.com/problems/minimum-falling-path-sum/
pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let mut dp = vec![0; m + 1];

    for row in (0..m).rev() {
        let mut curr_row = vec![0; m + 1];

        for col in 0..m {
            if col == 0 {
                curr_row[col] = dp[col].min(dp[col + 1]) + matrix[row][col];
            } else if (col == m - 1) {
                curr_row[col] = dp[col].min(dp[col - 1]) + matrix[row][col];
            } else {
                curr_row[col] = dp[col].min(dp[col + 1].min(dp[col - 1])) + matrix[row][col];
            }
        }

        dp = curr_row;
    }

    let mut min_sum = i32::MAX;
    for col in 0..m {
        min_sum = min_sum.min(dp[col]);
    }
    min_sum
}
