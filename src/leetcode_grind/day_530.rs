// https://leetcode.com/problems/longest-ideal-subsequence/description/
pub fn longest_ideal_string(s: String, k: i32) -> i32 {
    let n = s.len();
    let mut dp = vec![0; 150];
    for (i, ch) in s.chars().enumerate() {
        let curr = ch as i32 - 'a' as i32;
        let mut best = 0;
        for prev in 0..26 {
            if ((curr - prev).abs() <= k) {
                best = best.max(dp[prev as usize]);
            }
        }
        dp[curr as usize] = dp[curr as usize].max(best + 1);
    }
    dp.into_iter().max().unwrap_or(1)
}
