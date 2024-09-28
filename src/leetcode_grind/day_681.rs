// https://leetcode.com/problems/check-whether-two-strings-are-almost-equivalent/description/
pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let mut freq = vec![0; 26];
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = 0;
    while i < w1.len() || j < w2.len() {
        if i < w1.len() {
            freq[w1[i] as usize - 'a' as usize] += 1;
            i += 1;
        }
        if j < w2.len() {
            freq[w2[j] as usize - 'a' as usize] -= 1;
            j += 1;
        }
    }
    for x in freq {
        if x > 3 || x < -3 {
            return false;
        }
    }
    true
}
