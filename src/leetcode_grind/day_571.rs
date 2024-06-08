// https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/description
pub fn number_of_substrings(s: String) -> i64 {
    use std::collections::HashMap;
    let mut cnt = HashMap::new();
    let mut ans = 0;
    for ch in s.chars() {
        ans += 1;
        if cnt.contains_key(&ch) {
            ans += *cnt.get(&ch).unwrap();
        }
        *cnt.entry(ch).or_insert(0) += 1;
    }
    ans
}
