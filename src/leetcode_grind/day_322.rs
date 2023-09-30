use std::hash::Hash;

use crate::leetcode_grind::day_119::TreeNode;

// https://leetcode.com/problems/132-pattern/description
pub fn find132pattern_search_interval(nums: Vec<i32>) -> bool {
    let mut intervals = vec![];
    let mut s = 0;

    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            if s < i - 1 {
                intervals.push((s, i - 1));
            }
            s = i;
        }

        for a in &intervals {
            if nums[i] < nums[a.1] && nums[a.0] < nums[i] {
                return true;
            }
        }
    }
    false
}

pub fn find132pattern_stacks(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }
    let mut stack = vec![];

    let mut min = vec![];
    min.push(nums[0]);
    for i in 1..nums.len() {
        min.push(min[i - 1].min(nums[i]));
    }

    for j in (0..nums.len()).rev() {
        if nums[j] > min[j] {
            while !stack.is_empty() && stack[stack.len() - 1] <= min[j] {
                stack.pop();
            }
            if !stack.is_empty() && stack[stack.len() - 1] < nums[j] {
                return true;
            }
            stack.push(nums[j]);
        }
    }
    false
}

// https://leetcode.com/problems/word-search-ii/description/
pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;

    struct TrieNode {
        children: HashMap<char, TrieNode>,
        word: Option<String>,
    }

    impl TrieNode {
        fn new(words: Vec<String>) -> TrieNode {
            let mut trie = TrieNode {
                children: HashMap::new(),
                word: None,
            };
            for word in words {
                trie.insert(word);
            }
            trie
        }
        fn insert(&mut self, word: String) {
            let mut node = self;
            for ch in word.chars() {
                node.children.entry(ch).or_insert(TrieNode {
                    children: HashMap::new(),
                    word: None,
                });
                node = node.children.get_mut(&ch).unwrap();
            }
            node.word = Some(word);
        }
    }

    fn next(start: (usize, usize), dim: (i32, i32)) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for d in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (start.0 as i32 + d.0, start.1 as i32 + d.1);
            if nx >= 0 && nx < dim.0 && ny >= 0 && ny < dim.1 {
                result.push((nx as usize, ny as usize));
            }
        }
        result
    }

    fn dfs(
        trie: &mut TrieNode,
        board: &mut Vec<Vec<char>>,
        start: (usize, usize),
        result: &mut Vec<String>,
    ) {
        if let Some(word) = &trie.word {
            result.push(word.clone());
            trie.word = None;
        }

        let c = board[start.0][start.1];
        board[start.0][start.1] = '#';

        for nxt in next(start, (board.len() as i32, board[0].len() as i32)) {
            let ch = board[nxt.0][nxt.1];
            if trie.children.contains_key(&ch) {
                dfs(trie.children.get_mut(&ch).unwrap(), board, nxt, result);
            }
        }

        board[start.0][start.1] = c;
    }

    let mut board = board;
    let mut trie = TrieNode::new(words);
    let mut result = vec![];

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if trie.children.contains_key(&board[i][j]) {
                dfs(
                    trie.children.get_mut(&board[i][j]).unwrap(),
                    &mut board,
                    (i, j),
                    &mut result,
                );
            }
        }
    }
    result
}

#[test]
fn test_find_words() {
    let res = find_words(vec![vec!['a']], vec!["a".to_string()]);
    println!("{:?}", res);
}
