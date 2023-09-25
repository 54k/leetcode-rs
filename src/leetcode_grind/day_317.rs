// https://leetcode.com/problems/find-the-difference/description
pub fn find_the_difference(s: String, t: String) -> char {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    let mut t = t.chars().collect::<Vec<_>>();
    t.sort();

    for i in 0..s.len() {
        if s[i] != t[i] {
            return t[i];
        }
    }
    t[t.len() - 1]
}

// https://leetcode.com/problems/map-sum-pairs/description/
mod map_sum_bruteforce {
    use std::collections::HashMap;
    struct MapSum {
        map: HashMap<String, i32>,
    }

    impl MapSum {
        fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }

        fn insert(&mut self, key: String, val: i32) {
            self.map.insert(key, val);
        }

        fn sum(&self, prefix: String) -> i32 {
            let mut sum = 0;
            for (k, v) in &self.map {
                if k.starts_with(&prefix) {
                    sum += v;
                }
            }
            sum
        }
    }
}

mod map_sum_hash_map {
    use std::collections::HashMap;
    struct MapSum {
        map: HashMap<String, i32>,
        score: HashMap<String, i32>,
    }

    impl MapSum {
        fn new() -> Self {
            Self {
                map: HashMap::new(),
                score: HashMap::new(),
            }
        }

        fn insert(&mut self, key: String, val: i32) {
            let delta = val - *self.map.get(&key).unwrap_or(&0);
            self.map.insert(key.clone(), val);

            let mut prefix = "".to_string();
            for ch in key.chars() {
                prefix.push(ch);
                let prev = *self.score.get(&prefix).unwrap_or(&0);
                self.score.insert(prefix.clone(), prev + delta);
            }
        }

        fn sum(&self, prefix: String) -> i32 {
            *self.score.get(&prefix).unwrap_or(&0)
        }
    }
}

mod map_sum_trie {
    use std::collections::HashMap;

    struct TrieNode {
        children: HashMap<char, TrieNode>,
        score: i32,
    }
    impl TrieNode {
        fn new() -> Self {
            Self {
                children: HashMap::new(),
                score: 0,
            }
        }
    }

    struct MapSum {
        map: HashMap<String, i32>,
        root: TrieNode,
    }

    impl MapSum {
        fn new() -> Self {
            Self {
                map: HashMap::new(),
                root: TrieNode::new(),
            }
        }

        fn insert(&mut self, key: String, val: i32) {
            let delta = val - *self.map.get(&key).unwrap_or(&0);
            self.map.insert(key.clone(), val);

            let mut cur = &mut self.root;
            cur.score += delta;

            for ch in key.chars() {
                cur.children.entry(ch).or_insert(TrieNode::new());
                cur = cur.children.get_mut(&ch).unwrap();
                cur.score += delta;
            }
        }

        fn sum(&self, prefix: String) -> i32 {
            let mut cur = &self.root;
            for ch in prefix.chars() {
                let next = cur.children.get(&ch);
                if next.is_none() {
                    return 0;
                }
                cur = next.unwrap();
            }
            cur.score
        }
    }
}

// https://leetcode.com/problems/replace-words/description/
pub fn replace_words_prefix_set(dictionary: Vec<String>, sentence: String) -> String {
    use std::collections::HashSet;
    let mut prefixes = HashSet::new();

    for w in dictionary {
        prefixes.insert(w);
    }

    let mut ans = vec![];

    for w in sentence.split(" ") {
        let mut s = String::new();
        for ch in w.chars() {
            s.push(ch);
            if prefixes.contains(&s) {
                break;
            }
        }
        ans.push(s.to_string());
    }

    ans.join(" ")
}

pub fn replace_words_trie(dictionary: Vec<String>, sentence: String) -> String {
    use std::collections::HashMap;

    struct TrieNode {
        children: HashMap<char, TrieNode>,
        is_end: bool,
    }

    impl TrieNode {
        fn new() -> Self {
            Self {
                children: HashMap::new(),
                is_end: false,
            }
        }

        fn add(&mut self, key: &str) {
            let mut cur = self;
            for ch in key.chars() {
                cur.children.entry(ch).or_insert(TrieNode::new());
                cur = cur.children.get_mut(&ch).unwrap();
            }
            cur.is_end = true;
        }

        fn find_root(&self, key: &str) -> String {
            let mut ans = "".to_string();
            let mut cur = self;
            for ch in key.chars() {
                if cur.children.contains_key(&ch) {
                    cur = cur.children.get(&ch).unwrap();
                    ans.push(ch);
                    if cur.is_end {
                        return ans;
                    }
                } else {
                    return key.to_string();
                }
            }

            if cur.is_end {
                return ans;
            }
            return key.to_string();
        }
    }

    let mut ans = vec![];
    let mut root = TrieNode::new();
    for w in &dictionary {
        root.add(w);
    }

    for w in sentence.split(" ") {
        ans.push(root.find_root(w));
    }

    ans.join(" ")
}
