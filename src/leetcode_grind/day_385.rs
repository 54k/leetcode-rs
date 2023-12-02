// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/description
pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut count = vec![0; 26];
    for ch in chars.chars() {
        count[ch as usize - 'a' as usize] += 1;
    }
    let mut ans = 0;
    'outer: for word in words {
        let mut cc = count.clone();
        for ch in word.clone().chars() {
            if cc[ch as usize - 'a' as usize] == 0 {
                continue 'outer;
            } else {
                cc[ch as usize - 'a' as usize] -= 1;
            }
        }
        ans += word.len() as i32;
    }
    ans
}
