// https://leetcode.com/problems/palindromic-substrings/description
pub fn count_substrings(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![vec![false; n]; n];
    let mut ans = s.len() as i32;
    for i in (0..n).rev() {
        for j in i + 1..n {
            if s[i] == s[j] && (j - i <= 2 || dp[i + 1][j - 1]) {
                dp[i][j] = true;
            }
            ans += if dp[i][j] { 1 } else { 0 };
        }
    }
    ans
}
