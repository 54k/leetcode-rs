// https://leetcode.com/problems/longest-palindrome/description
pub fn longest_palindrome(s: String) -> i32 {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let mut res = 0;
    for c in s.as_bytes() {
        if set.contains(&c) {
            set.remove(&c);
            res += 2;
        } else {
            set.insert(c);
        }
    }
    if !set.is_empty() {
        res += 1;
    }
    res
}
