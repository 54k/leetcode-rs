// https://leetcode.com/problems/extra-characters-in-a-string/description/
pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    pub fn min_extra_char_top_down(s: String, dictionary: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut memo = vec![-1; s.len()];
        let dict = dictionary.into_iter().collect::<HashSet<_>>();

        fn dp(
            start: usize,
            n: usize,
            s: String,
            memo: &mut Vec<i32>,
            dict: &HashSet<String>,
        ) -> i32 {
            if start == n {
                return 0;
            }
            if memo[start] != -1 {
                return memo[start];
            }

            let mut ans = dp(start + 1, n, s.clone(), memo, dict) + 1;
            for end in start..n {
                let curr = s[start..=end].to_string();
                if dict.contains(&curr) {
                    ans = ans.min(dp(end + 1, n, s.clone(), memo, dict));
                }
            }

            memo[start] = ans;
            ans
        }

        dp(0, s.len(), s, &mut memo, &dict)
    }

    pub fn min_extra_char_bottom_up(s: String, dictionary: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let n = s.len();
        let mut dp = vec![0; s.len() + 1];
        let dict = dictionary.into_iter().collect::<HashSet<_>>();

        for start in (0..n).rev() {
            dp[start] = dp[start + 1] + 1;
            for end in start..n {
                let curr = s[start..=end].to_string();
                if dict.contains(&curr) {
                    dp[start] = dp[start].min(dp[end + 1]);
                }
            }
        }

        dp[0]
    }

    min_extra_char_bottom_up(s, dictionary)
}
