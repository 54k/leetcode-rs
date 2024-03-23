// https://leetcode.com/problems/remove-vowels-from-a-string/description/
pub fn remove_vowels(s: String) -> String {
    let mut res = String::new();
    for ch in s.chars() {
        if !['a', 'e', 'i', 'o', 'u'].contains(&ch) {
            res.push(ch);
        }
    }
    return res;
}
