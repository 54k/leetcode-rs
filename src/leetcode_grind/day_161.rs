// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/description/
pub fn min_insertions(s: String) -> i32 {
    fn lcs(s1: &Vec<char>, s2: &Vec<char>) -> i32 {
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
                }
            }
        }
        dp[s1.len()][s2.len()]
    }
    let s1 = s.chars().collect::<Vec<_>>();
    let mut s2 = s1.clone();
    s2.reverse();
    s1.len() as i32 - lcs(&s1, &s2)
}
