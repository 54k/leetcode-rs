// https://leetcode.com/problems/total-appeal-of-a-string/description/
pub fn appeal_sum(s: String) -> i64 {
    let mut ans = 0;
    let mut total = 0i64;
    let mut prev = vec![0; 26];
    for (i, ch) in s.chars().enumerate() {
        total += i as i64 + 1 - prev[ch as usize - 'a' as usize];
        prev[ch as usize - 'a' as usize] = i as i64 + 1;
        ans += total;
    }
    ans
}
