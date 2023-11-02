// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/description
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let (mut a, mut b) = (cost[0], cost[1]);
    for i in 2..cost.len() {
        let c = a.min(b) + cost[i];
        a = b;
        b = c;
    }
    a.min(b)
}

// https://leetcode.com/problems/minimum-path-sum/description/
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if i == 0 && j > 0 {
                grid[i][j] += grid[i][j - 1];
            } else if j == 0 && i > 0 {
                grid[i][j] += grid[i - 1][j];
            }
        }
    }
    for i in 1..grid.len() {
        for j in 1..grid[0].len() {
            grid[i][j] += grid[i - 1][j].min(grid[i][j - 1])
        }
    }
    grid[grid.len() - 1][grid[0].len() - 1]
}

// https://leetcode.com/problems/coin-change/description/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![100_000; amount as usize + 1];
    dp[0] = 0;

    for i in 1..=amount as usize {
        for j in 0..coins.len() {
            let d = coins[j] as usize;
            if d <= i {
                dp[i] = dp[i].min(dp[i - d] + 1);
            }
        }
    }
    
    if dp[amount as usize] == 100_000 {
        return -1;
    }
    dp[amount as usize]
}
