// https://leetcode.com/problems/determine-if-two-strings-are-close/description/
pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut f1 = vec![0; 26];
    let mut f2 = vec![0; 26];
    for c in word1.chars() {
        f1[c as usize - 'a' as usize] += 1;
    }
    for c in word2.chars() {
        f2[c as usize - 'a' as usize] += 1;
    }

    for i in 0..26 {
        if f1[i] == 0 && f2[i] > 0 || f2[i] == 0 && f1[i] > 0 {
            return false;
        }
    }
    f1.sort();
    f2.sort();
    f1 == f2
}
