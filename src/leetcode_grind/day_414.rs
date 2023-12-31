// https://leetcode.com/problems/largest-substring-between-two-equal-characters/description
pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut arr = vec![-1; 26];
    let mut ans = -1;
    for (i, ch) in s.chars().enumerate() {
        let k = ch as usize - 'a' as usize;
        if arr[k] > -1 {
            ans = ans.max(i as i32 - arr[k] - 1);
        } else {
            arr[k] = i as i32;
        }
    }
    ans
}
