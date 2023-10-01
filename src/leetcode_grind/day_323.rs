// https://leetcode.com/problems/reverse-words-in-a-string-iii/
pub fn reverse_words_traverse_reverse(s: String) -> String {
    let mut ans = vec![];
    let s = s.chars().collect::<Vec<_>>();

    let mut last_space_index = -1;
    for str_index in 0..s.len() {
        if str_index == s.len() - 1 || s[str_index] == ' ' {
            let mut reverse_str_index = if str_index == s.len() - 1 {
                str_index as i32
            } else {
                str_index as i32 - 1
            };

            while reverse_str_index > last_space_index {
                ans.push(s[reverse_str_index as usize]);
                reverse_str_index -= 1;
            }

            if str_index != s.len() - 1 {
                ans.push(' ');
            }

            last_space_index = str_index as i32;
        }
    }
    ans.into_iter().collect::<String>()
}

pub fn reverse_words_two_pointers(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut word_start = 0;
    for i in 0..s.len() {
        if i == s.len() - 1 || s[i] == ' ' {
            let mut word_end = if i == s.len() - 1 { s.len() - 1 } else { i - 1 };
            while word_start < word_end {
                let tmp = s[word_start];
                s[word_start] = s[word_end];
                s[word_end] = tmp;
                word_start += 1;
                word_end -= 1;
            }
            word_start = i + 1;
        }
    }
    s.into_iter().collect()
}

// https://leetcode.com/problems/word-squares/description/
pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    struct TrieNode {
        children: HashMap<char, TrieNode>,
        words: Vec<String>,
    }
    impl TrieNode {
        fn new(words: Vec<String>) -> Self {
            let mut root = Self {
                children: HashMap::new(),
                words: vec![],
            };
            for word in words {
                root.insert(word)
            }
            root
        }

        fn insert(&mut self, word: String) {
            let mut node = self;
            for ch in word.chars() {
                node.children.entry(ch).or_insert(TrieNode {
                    children: HashMap::new(),
                    words: vec![],
                });
                node = node.children.get_mut(&ch).unwrap();
                node.words.push(word.clone());
            }
        }

        fn find(&mut self, prefix: String) -> Vec<String> {
            let mut node = self;
            for ch in prefix.chars() {
                node.children.entry(ch).or_insert(TrieNode {
                    children: HashMap::new(),
                    words: vec![],
                });
                node = node.children.get_mut(&ch).unwrap();
            }
            node.words.clone()
        }
    }

    fn backtrack(
        step: usize,
        len: usize,
        cur: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
        trie: &mut TrieNode,
    ) {
        if step == len {
            result.push(cur.clone());
            return;
        }

        let mut prefix = String::new();
        for w in cur.iter() {
            prefix.push(w.chars().nth(step).unwrap());
        }

        for word in trie.find(prefix) {
            cur.push(word);
            backtrack(step + 1, len, cur, result, trie);
            cur.pop();
        }
    }

    let len = words[0].len();
    let mut trie = TrieNode::new(words.clone());
    let mut result = vec![];
    for word in words {
        let mut cur = vec![word];
        backtrack(1, len, &mut cur, &mut result, &mut trie)
    }
    result
}
