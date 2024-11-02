// https://leetcode.com/problems/circular-sentence/description/?envType=daily-question&envId=2024-11-02
pub fn is_circular_sentence(sentence: String) -> bool {
    let sentence = sentence.chars().collect::<Vec<_>>();
    if sentence[0] != sentence[sentence.len() - 1] {
        return false;
    }
    for i in 1..sentence.len() - 1 {
        if sentence[i] == ' ' && sentence[i - 1] != sentence[i + 1] {
            return false;
        }
    }
    true
}
