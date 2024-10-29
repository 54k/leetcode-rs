// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/description/?envType=daily-question&envId=2024-10-29
pub fn max_moves_i(grid: Vec<Vec<i32>>) -> i32 {
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

pub fn max_moves_ii(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut dp = vec![vec![0; 2]; m];
    for i in 0..m {
        dp[i][0] = 1;
    }
    let mut max_moves = 0;
    for j in 1..n {
        for i in 0..m {
            if grid[i][j] > grid[i][j - 1] && dp[i][0] > 0 {
                dp[i][1] = dp[i][1].max(dp[i][0] + 1);
            }
            if i >= 1 && grid[i][j] > grid[i - 1][j - 1] && dp[i - 1][0] > 0 {
                dp[i][1] = dp[i][1].max(dp[i - 1][0] + 1);
            }
            if i + 1 < m && grid[i][j] > grid[i + 1][j - 1] && dp[i + 1][0] > 0 {
                dp[i][1] = dp[i][1].max(dp[i + 1][0] + 1);
            }

            max_moves = max_moves.max(dp[i][1] - 1);
        }

        for k in 0..m {
            dp[k][0] = dp[k][1];
            dp[k][1] = 0;
        }
    }

    max_moves
}

// https://leetcode.com/problems/paint-house-ii/description/?envType=weekly-question&envId=2024-10-29
pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
    let (n, m) = (costs.len(), costs[0].len());
    let mut dp = vec![vec![1000_000_000; m]; n + 1];
    dp[0] = vec![0; m];
    for i in 1..=n {
        for j in 0..m {
            for k in 0..m {
                if j == k {
                    continue;
                }
                dp[i][j] = dp[i][j].min(costs[i - 1][j] + dp[i - 1][k])
            }
        }
    }
    dp[n].iter().copied().min().unwrap()
}
