// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/description/?envType=daily-question&envId=2024-10-29
pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut dp = vec![vec![0; n]; m];
    for i in 0..m {
        dp[i][0] = 1;
    }

    let mut max_moves = 0;
    for j in 1..n {
        for i in 0..m {
            if grid[i][j] > grid[i][j - 1] && dp[i][j - 1] > 0 {
                dp[i][j] = dp[i][j].max(dp[i][j - 1] + 1);
            }
            if i >= 1 && grid[i][j] > grid[i - 1][j - 1] && dp[i - 1][j - 1] > 0 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
            }
            if i + 1 < m && grid[i][j] > grid[i + 1][j - 1] && dp[i + 1][j - 1] > 0 {
                dp[i][j] = dp[i][j].max(dp[i + 1][j - 1] + 1);
            }
            max_moves = max_moves.max(dp[i][j] - 1);
        }
    }

    max_moves
}
