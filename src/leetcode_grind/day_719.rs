// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/description/?envType=daily-question&envId=2024-11-05
pub fn min_changes(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = 0;
    for i in (0..s.len()).step_by(2) {
        if s[i] != s[i + 1] {
            ans += 1;
        }
    }
    ans
}
