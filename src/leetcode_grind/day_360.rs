// https://leetcode.com/problems/palindrome-partitioning-iv/description/
pub fn check_partitioning(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let mut palindrome_dp = vec![vec![false; s.len()]; s.len()];
    for end in 0..s.len() {
        for start in 0..=end {
            if s[start] == s[end] && (end - start <= 2 || palindrome_dp[start + 1][end - 1]) {
                palindrome_dp[start][end] = true;
            }
        }
    }

    for i in 1..s.len() - 1 {
        for j in i..s.len() - 1 {
            if palindrome_dp[0][i - 1] && palindrome_dp[i][j] && palindrome_dp[j + 1][s.len() - 1] {
                return true;
            }
        }
    }
    false
}
