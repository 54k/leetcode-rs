// https://leetcode.com/problems/relative-ranks/description
pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    use std::collections::HashMap;
    let mut orig = score;
    let mut score = orig.clone();
    let mut hm = HashMap::new();
    score.sort();
    for (i, &s) in score.iter().rev().enumerate() {
        hm.insert(s, i + 1);
    }
    let mut ans = vec![];
    for s in orig.iter() {
        let sc = *hm.get(s).unwrap();
        if sc == 1 {
            ans.push("Gold Medal".to_string());
        } else if sc == 2 {
            ans.push("Silver Medal".to_string());
        } else if sc == 3 {
            ans.push("Bronze Medal".to_string());
        } else {
            ans.push(format!("{sc}"));
        }
    }
    ans
}
