// https://leetcode.com/problems/is-subsequence/editorial
pub fn is_subsequence_two_pointers(s: String, t: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut left = 0;
    let mut right = 0;
    while left < s.len() && right < t.len() {
        if s[left] == t[right] {
            left += 1;
        }
        right += 1;
    }
    left == s.len()
}
