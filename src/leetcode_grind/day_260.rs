// https://leetcode.com/problems/strange-printer/description/
pub fn strange_printer(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![vec![n as i32; n]; n];
    for length in 1..=n {
        for left in 0..=n - length {
            let right = left + length - 1;
            let mut j = -1;

            for i in left..right {
                if s[i] != s[right] && j == -1 {
                    j = i as i32;
                }
                if j != -1 {
                    dp[left][right] =
                        dp[left][right].min(1 + dp[j as usize][i] + dp[i + 1][right]) as i32;
                }
            }

            if j == -1 {
                dp[left][right] = 0;
            }
        }
    }

    dp[0][n - 1] + 1
}
