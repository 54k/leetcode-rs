// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/description
pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let word1 = word1
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let word2 = word2
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut w1 = 0;
    let mut w2 = 0;
    let mut s1 = 0;
    let mut s2 = 0;

    while s1 < word1.len() && s2 < word2.len() {
        if word1[s1][w1] != word2[s2][w2] {
            return false;
        }

        w1 += 1;
        w2 += 1;

        if w1 == word1[s1].len() {
            w1 = 0;
            s1 += 1;
        }

        if w2 == word2[s2].len() {
            w2 = 0;
            s2 += 1;
        }
    }

    s1 == word1.len() && s2 == word2.len()
}
