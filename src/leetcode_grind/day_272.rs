// https://leetcode.com/problems/alien-dictionary/
pub fn alien_order(words: Vec<String>) -> String {
    use std::collections::{HashMap, VecDeque};
    let mut adj = HashMap::new();
    let mut count = HashMap::new();

    for word in &words {
        for ch in word.chars() {
            count.entry(ch).or_insert(0);
            adj.entry(ch).or_insert(vec![]);
        }
    }

    for i in 0..words.len() - 1 {
        let word1 = words[i].clone().chars().collect::<Vec<_>>();
        let word2 = words[i + 1].clone().chars().collect::<Vec<_>>();

        if word1.len() > word2.len() && word1.starts_with(&word2) {
            return "".to_string();
        }

        for j in 0..word1.len().min(word2.len()) {
            if word1[j] != word2[j] {
                adj.entry(word1[j]).or_insert(vec![]).push(word2[j]);
                *count.entry(word2[j]).or_insert(0) += 1;
                break;
            }
        }
    }

    let mut queue = VecDeque::new();
    for (k, &v) in &count {
        if v == 0 {
            queue.push_back(*k);
        }
    }
    let mut ans = vec![];
    while let Some(ch) = queue.pop_front() {
        ans.push(ch);
        for &next in &adj[&ch] {
            *count.entry(next).or_insert(0) -= 1;
            if count[&next] == 0 {
                queue.push_back(next);
            }
        }
    }
    if ans.len() != count.len() {
        return "".to_string();
    }
    ans.into_iter().collect()
}
