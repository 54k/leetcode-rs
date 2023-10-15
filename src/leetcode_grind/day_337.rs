// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/
pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
    const MOD: i32 = 1000000007;
    let arr_len = arr_len.min(steps) as usize;
    let steps = steps as usize;
    let mut dp = vec![vec![0; steps + 1]; arr_len];
    dp[0][0] = 1;

    for remain in 1..=steps {
        for curr in (0..arr_len).rev() {
            dp[curr][remain] = (dp[curr][remain] + dp[curr][remain - 1]) % MOD;
            if curr > 0 {
                dp[curr][remain] = (dp[curr][remain] + dp[curr - 1][remain - 1]) % MOD;
            }
            if curr < arr_len - 1 {
                dp[curr][remain] = (dp[curr][remain] + dp[curr + 1][remain - 1]) % MOD;
            }
        }
    }

    dp[0][steps as usize]
}

// https://leetcode.com/problems/painting-the-walls/description/
pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    const INF: i32 = 1000000000;
    let n = cost.len();

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        dp[n][i] = INF;
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
