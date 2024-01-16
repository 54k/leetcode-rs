// https://leetcode.com/problems/number-of-segments-in-a-string/
pub fn count_segments(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..s.len() {
        if (i == 0 || s[i - 1] == ' ') && s[i] != ' ' {
            ans += 1;
        }
    }
    ans
}
