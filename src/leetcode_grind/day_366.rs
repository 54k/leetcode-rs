// https://leetcode.com/problems/sort-vowels-in-a-string/description/
pub fn sort_vowels(s: String) -> String {
    let mut vowels = vec![];
    for ch in s.chars() {
        if ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&ch) {
            vowels.push(ch);
        }
    }
    vowels.sort();
    let mut v_i = 0;
    let mut s = s.chars().collect::<Vec<_>>();
    for i in 0..s.len() {
        if ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&s[i]) {
            s[i] = vowels[v_i];
            v_i += 1;
        }
    }
    s.into_iter().collect()
}
