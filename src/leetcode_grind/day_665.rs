// https://leetcode.com/problems/count-the-number-of-consistent-strings/description/?envType=daily-question&envId=2024-09-12
pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let allowed = allowed.chars().collect::<HashSet<_>>();
    let mut ans = 0;
    'lb: for w in words {
        let w = w.chars().collect::<HashSet<_>>();
        for c in &w {
            if !allowed.contains(c) {
                continue 'lb;
            }
        }
        ans += 1;
    }
    ans
}
