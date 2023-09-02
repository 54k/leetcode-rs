// https://leetcode.com/problems/extra-characters-in-a-string/description/
pub fn min_extra_char_i(s: String, dictionary: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let dict = dictionary.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![0; s.len() + 1];

    for start in (0..s.len()).rev() {
        dp[start] = dp[start + 1] + 1;
        for end in start..s.len() {
            if dict.contains(&s[start..=end]) {
                dp[start] = dp[start].min(dp[end + 1]);
            }
        }
    }
    dp[0]
}

pub fn min_extra_char_ii(s: String, dictionary: Vec<String>) -> i32 {
    use std::collections::HashMap;

    struct TrieNode {
        children: HashMap<char, TrieNode>,
        is_word: bool,
    }

    impl TrieNode {
        fn new() -> Self {
            Self {
                children: HashMap::new(),
                is_word: false,
            }
        }
    }

    fn build_trie(dictionary: Vec<String>) -> TrieNode {
        let mut root = TrieNode::new();
        for word in dictionary {
            let mut node = &mut root;

            for ch in word.chars() {
                node.children.entry(ch).or_insert(TrieNode::new());
                node = node.children.get_mut(&ch).unwrap();
            }

            node.is_word = true;
        }
        root
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![0; s.len() + 1];

    let mut root = build_trie(dictionary);

    for start in (0..s.len()).rev() {
        dp[start] = dp[start + 1] + 1;
        let mut node = &mut root;

        for end in start..s.len() {
            let ch = s[end];
            if !node.children.contains_key(&ch) {
                break;
            }
            node = node.children.get_mut(&ch).unwrap();
            if node.is_word {
                dp[start] = dp[start].min(dp[end + 1]);
            }
        }
    }

    dp[0]
}

// https://leetcode.com/problems/word-break/
pub fn word_break_i(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];
    dp[s.len()] = true;

    for i in (0..s.len()).rev() {
        for w in &word_dict {
            if s[i..].starts_with(w) {
                dp[i] |= dp[i + w.len()];
            }
        }
    }
    dp[0]
}

pub fn word_break_ii(s: String, word_dict: Vec<String>) -> bool {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    let dict = word_dict.into_iter().collect::<HashSet<_>>();
    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut seen = vec![false; s.len() + 1];

    while let Some(start) = queue.pop_front() {
        if start == s.len() {
            return true;
        }

        for end in start + 1..=s.len() {
            if seen[end] {
                continue;
            }

            if dict.contains(&s[start..end]) {
                queue.push_back(end);
                seen[end] = true;
            }
        }
    }

    false
}

pub fn word_break_iii(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len()];
    for end in 0..s.len() {
        for word in &word_dict {
            if end < word.len() - 1 {
                continue;
            }
            if end == word.len() - 1 || dp[end - word.len()] {
                if word == &s[end - word.len() + 1..end + 1].to_string() {
                    dp[end] = true;
                    break;
                }
            }
        }
    }
    dp[s.len() - 1]
}

pub fn word_break_iv(s: String, word_dict: Vec<String>) -> bool {
    use std::collections::HashSet;
    let dict = word_dict.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;
    for i in 1..=s.len() {
        for j in 0..i {
            if dp[j] && dict.contains(&s[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[s.len()]
}
