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

// https://leetcode.com/problems/nested-list-weight-sum/
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub fn depth_sum_i(nested_list: Vec<NestedInteger>) -> i32 {
    fn dfs(nested_list: &Vec<NestedInteger>, depth: i32) -> i32 {
        let mut total = 0;
        for nested in nested_list {
            match nested {
                NestedInteger::Int(num) => {
                    total += num * depth;
                }
                NestedInteger::List(next) => {
                    total += dfs(&next, depth + 1);
                }
            }
        }
        total
    }
    dfs(&nested_list, 1)
}

pub fn depth_sum_ii(nested_list: Vec<NestedInteger>) -> i32 {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    for list in nested_list {
        queue.push_back(list);
    }

    let mut total = 0;
    let mut depth = 1;

    while queue.len() > 0 {
        for _ in 0..queue.len() {
            let el = queue.pop_front().unwrap();
            match el {
                NestedInteger::Int(num) => {
                    total += num * depth;
                }
                NestedInteger::List(next) => {
                    for list in next {
                        queue.push_back(list);
                    }
                }
            }
        }
        depth += 1;
    }
    total
}

// https://leetcode.com/problems/nested-list-weight-sum-ii/
pub fn depth_sum_inverse_i(nested_list: Vec<NestedInteger>) -> i32 {
    fn get_weighted_sum_triplet(list: &Vec<NestedInteger>, depth: i32) -> (i32, i32, i32) {
        let mut sum_of_products = 0;
        let mut sum_of_elements = 0;
        let mut max_depth = 0;

        for nested in list {
            match nested {
                NestedInteger::Int(num) => {
                    sum_of_products += num * depth;
                    sum_of_elements += num;
                    max_depth = max_depth.max(depth);
                }
                NestedInteger::List(next) => {
                    if next.is_empty() {
                        max_depth = max_depth.max(depth);
                        continue;
                    }
                    let triplet = get_weighted_sum_triplet(&next, depth + 1);
                    sum_of_products += triplet.0;
                    sum_of_elements += triplet.1;
                    max_depth = max_depth.max(triplet.2);
                }
            }
        }

        (sum_of_products, sum_of_elements, max_depth)
    }

    let (sum_of_products, sum_of_elements, max_depth) = get_weighted_sum_triplet(&nested_list, 1);
    (max_depth + 1) * sum_of_elements - sum_of_products
}

pub fn depth_sum_inverse_ii(nested_list: Vec<NestedInteger>) -> i32 {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    for l in nested_list {
        queue.push_back(l);
    }

    let mut depth = 1;
    let mut max_depth = 0;
    let mut total_sum = 0;
    let mut total_product_sum = 0;

    while queue.len() > 0 {
        max_depth = max_depth.max(depth);
        for _ in 0..queue.len() {
            let el = queue.pop_front().unwrap();
            match el {
                NestedInteger::Int(num) => {
                    total_sum += num;
                    total_product_sum += num * depth;
                }
                NestedInteger::List(list) => {
                    for l in list {
                        queue.push_back(l);
                    }
                }
            }
        }
        depth += 1;
    }

    (max_depth + 1) * total_sum - total_product_sum
}
