// https://leetcode.com/problems/number-of-music-playlists/description/
pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
    const MOD: i64 = 1000000007;
    let mut dp = vec![vec![0i64; n as usize + 1]; goal as usize + 1];
    dp[0][0] = 1;
    for i in 1..=goal as usize {
        for j in 1..=(i as usize).min(n as usize) {
            dp[i][j] = (dp[i][j] + dp[i - 1][j - 1] * (n as i64 - j as i64 + 1)) % MOD;
            if j > k as usize {
                dp[i][j] = (dp[i][j] + dp[i - 1][j] * (j as i64 - k as i64)) % MOD;
            }
        }
    }
    dp[goal as usize][n as usize] as i32
}
