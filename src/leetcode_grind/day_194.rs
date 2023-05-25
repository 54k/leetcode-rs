// https://leetcode.com/problems/new-21-game/description/
pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    let mut dp = vec![0.; n as usize + 1];
    dp[0] = 1.;
    let mut s = if k > 0 { 1. } else { 0. };
    for i in 1..=n {
        dp[i as usize] = s / max_pts as f64;
        if i < k {
            s += dp[i as usize];
        }
        if i - max_pts >= 0 && i - max_pts < k {
            s -= dp[(i - max_pts) as usize];
        }
    }

    let mut ans = 0.;
    for i in k..=n {
        ans += dp[i as usize];
    }
    ans
}
