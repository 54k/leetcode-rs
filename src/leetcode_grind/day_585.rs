// https://leetcode.com/problems/count-substrings-without-repeating-character/description
pub fn number_of_special_substrings(s: String) -> i32 {
    let mut ans = 0;
    let mut freq = vec![0; 26];
    let s = s.chars().collect::<Vec<_>>();
    let mut start = 0;
    for end in 0..s.len() {
        freq[s[end] as usize - 'a' as usize] += 1;
        while freq[s[end] as usize - 'a' as usize] > 1 {
            freq[s[start] as usize - 'a' as usize] -= 1;
            start += 1;
        }
        ans += end - start + 1;
    }
    ans as i32
}
