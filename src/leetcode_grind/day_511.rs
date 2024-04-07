// https://leetcode.com/problems/valid-parenthesis-string/description/
pub fn check_valid_string(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![vec![false; n + 1]; n + 1];
    dp[n][0] = true;
    for i in (0..n).rev() {
        for bal in 0..n {
            let ch = s[i];
            if ch == '*' {
                if bal > 0 {
                    dp[i][bal] |= dp[i + 1][bal - 1];
                }
                dp[i][bal] |= dp[i + 1][bal + 1];
                dp[i][bal] |= dp[i + 1][bal];
            } else if ch == '(' {
                dp[i][bal] |= dp[i + 1][bal + 1];
            } else if bal > 0 {
                dp[i][bal] |= dp[i + 1][bal - 1];
            }
        }
    }
    dp[0][0]
}
pub mod day_511;