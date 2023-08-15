// https://leetcode.com/problems/longest-palindromic-substring/description/
pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![false; n]; n];
    let mut ans = vec![0, 0];

    for i in 0..n {
        dp[i][i] = true;
    }

    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = true;
            ans[0] = i;
            ans[1] = i + 1;
        }
    }

    for diff in 2..n {
        for i in 0..n - diff {
            let j = i + diff;
            if s[i] == s[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                ans[0] = i;
                ans[1] = j;
            }
        }
    }

    let i = ans[0];
    let j = ans[1];
    s[i..=j].into_iter().collect()
}
