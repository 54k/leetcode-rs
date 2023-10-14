// https://leetcode.com/problems/painting-the-walls/description
pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        dp[n][i] = 1000000000;
    }

    for i in (0..n).rev() {
        for remain in 1..=n {
            let paint = cost[i] + dp[i + 1][0.max(remain as i32 - 1 - time[i]) as usize];
            let dont_paint = dp[i + 1][remain];
            dp[i][remain] = paint.min(dont_paint);
        }
    }

    dp[0][n]
}
