// https://leetcode.com/problems/shortest-palindrome/description/?envType=daily-question&envId=2024-09-20
pub fn shortest_palindrome(s: String) -> String {
    fn build_prefix_table(s: String) -> Vec<usize> {
        let s = s.chars().collect::<Vec<_>>();
        let mut prefix_table = vec![0; s.len()];
        let mut length = 0;

        for i in 1..s.len() {
            while length > 0 && s[i] != s[length] {
                length = prefix_table[length - 1];
            }
            if s[i] == s[length] {
                length += 1;
            }
            prefix_table[i] = length;
        }

        prefix_table
    }

    let rev_s = s.chars().rev().collect::<String>();
    let comb_s = s.clone() + "#" + &rev_s;
    let mut prefix_table = build_prefix_table(comb_s.clone());
    let palindrome_length = prefix_table[comb_s.len() - 1];
    let suffix = s[palindrome_length..].chars().rev().collect::<String>();
    suffix + &s
}

// https://leetcode.com/problems/maximum-deletions-on-a-string/description/
// https://leetcode.com/problems/maximum-deletions-on-a-string/solutions/2648900/java-c-python-dp-solution/
pub fn delete_string(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut lcs = vec![vec![0; n + 1]; n + 1];
    let mut dp = vec![0; n];
    for i in (0..n).rev() {
        dp[i] = 1;
        for j in i + 1..n {
            if s[i] == s[j] {
                lcs[i][j] = lcs[i + 1][j + 1] + 1;
            }
            if lcs[i][j] >= j - i {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    dp[0]
}

// https://leetcode.com/problems/shortest-distance-to-a-character/description/
pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = vec![0; s.len()];
    let mut prev = i32::MIN / 2;
    for i in 0..s.len() {
        if s[i] == c {
            prev = i as i32;
        }
        ans[i] = i as i32 - prev;
    }
    prev = i32::MAX / 2;
    for i in (0..s.len()).rev() {
        if s[i] == c {
            prev = i as i32;
        }
        ans[i] = ans[i].min(prev - i as i32);
    }
    ans
}
