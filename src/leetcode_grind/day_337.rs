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
