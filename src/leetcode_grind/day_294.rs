// https://leetcode.com/problems/extra-characters-in-a-string/description/
pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let mut dict = dictionary.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![0; s.len() + 1];

    for start in (0..s.len()).rev() {
        dp[start] = dp[start + 1] + 1;
        for end in start..s.len() {
            if dict.contains(&s[start..=end]) {
                dp[start] = dp[start].min(dp[end + 1]);
            }
        }
    }

    dp[0]
}

// https://leetcode.com/problems/word-break/
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    todo!()
}
