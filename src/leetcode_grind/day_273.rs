// https://leetcode.com/problems/unique-paths-ii/description/
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    let mut dp = vec![0; n];
    dp[0] = 1;
    for row in 0..m {
        let mut next = vec![0; n];
        for col in 0..n {
            if obstacle_grid[row][col] == 0 {
                next[col] += dp[col];
                if col > 0 && obstacle_grid[row][col - 1] == 0 {
                    next[col] += next[col - 1];
                }
            }
        }
        dp = next;
    }
    dp[n - 1]
}
