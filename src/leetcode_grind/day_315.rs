// https://leetcode.com/problems/minimum-window-subsequence/description/
pub fn min_window(s1: String, s2: String) -> String {
    let (s1, s2) = (
        s1.chars().collect::<Vec<_>>(),
        s2.chars().collect::<Vec<_>>(),
    );
    let (n, m) = (s1.len(), s2.len());
    let mut dp = vec![vec![10e6 as i32; m + 1]; n + 1];
    dp[0][0] = 0;

    let mut end = 0;
    let mut length = n as i32 + 1;

    for i in 1..=n {
        dp[i][0] = 0;

        for j in 1..=m {
            dp[i][j] = 1 + if s1[i - 1] == s2[j - 1] {
                dp[i - 1][j - 1]
            } else {
                dp[i - 1][j]
            };
        }

        if dp[i][m] < length {
            length = dp[i][m];
            end = i as i32;
        }
    }

    if length > n as i32 {
        "".to_string()
    } else {
        s1[end as usize - length as usize..end as usize]
            .into_iter()
            .collect()
    }
}
