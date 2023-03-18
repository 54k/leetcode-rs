use std::cell::RefCell;
// https://leetcode.com/problems/lru-cache/description/
use crate::grind_169::week_3::find_anagrams;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ptr::null_mut;
use std::rc::Rc;

// https://leetcode.com/problems/minimum-height-trees/
// https://leetcode.com/problems/minimum-height-trees/editorial/
pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut n = n as usize;
    let mut adj = vec![HashSet::new(); n];

    for edge in edges {
        adj[edge[0] as usize].insert(edge[1] as usize);
        adj[edge[1] as usize].insert(edge[0] as usize);
    }

    let mut leaves = vec![];
    for (i, children) in adj.iter().enumerate() {
        if children.len() < 2 {
            // node with 0,1 children is a leaf
            leaves.push(i);
        }
    }

    while n > 2 {
        n -= leaves.len();
        let mut next_leaves = vec![];
        for &leave in &leaves {
            let neighbor = *adj[leave].iter().next().unwrap();
            adj[neighbor].remove(&leave);
            if adj[neighbor].len() == 1 {
                next_leaves.push(neighbor);
            }
        }
        leaves = next_leaves;
    }

    leaves.into_iter().map(|x| x as i32).collect()
}

// https://leetcode.com/problems/task-scheduler/description/
// https://medium.com/@satyem77/task-scheduler-leetcode-39d579f3440
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let total_jobs = tasks.len();
    let mut freqs = vec![0; 26];
    let mut max_freq = 0;
    for t in tasks {
        let f = &mut freqs[t as usize - 'A' as usize];
        *f += 1;
        if max_freq < *f {
            max_freq = *f;
        }
    }
    let mut max_freq_count = 0;
    for f in freqs {
        if f == max_freq {
            max_freq_count += 1;
        }
    }
    (total_jobs as i32).max((n + 1) * (max_freq - 1) + max_freq_count)
}

struct ListNode {
    val: i32,
    key: i32,
    prev: *mut ListNode,
    next: *mut ListNode,
}
impl ListNode {
    fn evict(&mut self) -> *mut ListNode {
        unsafe {
            let prev = self.prev;
            let next = self.next;
            (*prev).next = next;
            (*next).prev = prev;
            self
        }
    }
}

struct DoublyLinkedList {
    head: *mut ListNode,
    tail: *mut ListNode,
}
impl DoublyLinkedList {
    fn new() -> Self {
        unsafe {
            let dummy_head = Box::into_raw(Box::new(ListNode {
                key: i32::MIN,
                val: i32::MIN,
                prev: null_mut(),
                next: null_mut(),
            }));
            let dummy_tail = Box::into_raw(Box::new(ListNode {
                key: i32::MAX,
                val: i32::MAX,
                prev: null_mut(),
                next: null_mut(),
            }));
            (*dummy_head).prev = dummy_head;
            (*dummy_head).next = dummy_tail;
            (*dummy_tail).prev = dummy_head;
            (*dummy_tail).next = dummy_tail;
            Self {
                head: dummy_head,
                tail: dummy_tail,
            }
        }
    }
    fn push_back(&self, key: i32, val: i32) -> *mut ListNode {
        unsafe {
            let new_node = Box::into_raw(Box::new(ListNode {
                key,
                val,
                prev: null_mut(),
                next: null_mut(),
            }));
            let prev = (*self.tail).prev;
            (*new_node).prev = prev;
            (*new_node).next = self.tail;
            (*prev).next = new_node;
            (*self.tail).prev = new_node;
            new_node
        }
    }
    fn pop_front(&self) -> *mut ListNode {
        unsafe {
            let next = (*self.head).next;
            (*next).evict()
        }
    }
}

struct LRUCache {
    key_to_node: HashMap<i32, *mut ListNode>,
    linked_list_lru: DoublyLinkedList,
    capacity: usize,
}
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            key_to_node: HashMap::new(),
            linked_list_lru: DoublyLinkedList::new(),
            capacity: capacity as usize,
        }
    }
    fn get(&mut self, key: i32) -> i32 {
        if !self.key_to_node.contains_key(&key) {
            -1
        } else {
            unsafe {
                let list_node_entry = self.key_to_node.get_mut(&key).unwrap();
                let list_node = *list_node_entry;
                let val = (*(*list_node).evict()).val;
                *list_node_entry = self.linked_list_lru.push_back(key, val);
                val
            }
        }
    }
    fn put(&mut self, key: i32, value: i32) {
        unsafe {
            if !self.key_to_node.contains_key(&key) && self.capacity == self.key_to_node.len() {
                let list_node = self.linked_list_lru.pop_front();
                self.key_to_node.remove(&(*list_node).key).unwrap();
                let _ = Box::from_raw(list_node); // to let you die
            }
            if let std::collections::hash_map::Entry::Vacant(e) = self.key_to_node.entry(key) {
                let list_node = self.linked_list_lru.push_back(key, value);
                e.insert(list_node);
            } else {
                (**self.key_to_node.get_mut(&key).unwrap()).val = value;
                self.get(key);
            }
        }
    }
}

// https://leetcode.com/problems/kth-smallest-element-in-a-bst/description/
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/editorial/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn iterative(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack = vec![];
        loop {
            while root.is_some() {
                stack.push(root.clone());
                let left = root.as_ref().unwrap().borrow().left.clone();
                root = left;
            }
            if let Some(Some(r)) = stack.pop() {
                let r = r.borrow();
                k -= 1;
                if k == 0 {
                    return r.val;
                }
                root = r.right.clone();
            }
        }
    }
    fn recursive(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, ans: &mut i32) {
            if let Some(r) = root {
                let r = r.borrow();
                inorder(r.left.clone(), k, ans);
                *k -= 1;
                if *k == 0 {
                    *ans = r.val;
                    return;
                }
                inorder(r.right.clone(), k, ans);
            }
        }
        let mut ans = 0;
        inorder(root, &mut k, &mut ans);
        ans
    }
    assert_eq!(iterative(root.clone(), k), recursive(root.clone(), k));
    iterative(root, k)
}

// https://leetcode.com/problems/daily-temperatures/
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; temperatures.len()];
    let mut mono_stack = vec![];
    for i in 0..temperatures.len() {
        while !mono_stack.is_empty()
            && temperatures[mono_stack[mono_stack.len() - 1]] < temperatures[i]
        {
            let j = mono_stack.pop().unwrap();
            ans[j] = (i - j) as i32;
        }
        mono_stack.push(i);
    }
    ans
}

// https://leetcode.com/problems/house-robber/description/
pub fn rob(nums: Vec<i32>) -> i32 {
    fn with_dp_memo(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        if nums.len() == 1 {
            return nums[0];
        }
        dp[0] = nums[0];
        dp[1] = nums[1].max(nums[0]);
        for i in 2..nums.len() {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        dp[nums.len() - 1]
    }
    fn memory_reduced(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut last = 0;
        let mut ans = 0;
        for n in nums {
            ans = last.max(prev + n);
            prev = last;
            last = ans;
        }
        ans
    }
    memory_reduced(nums)
}

// https://leetcode.com/problems/gas-station/
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total_gas = 0;
    let mut total_cost = 0;
    let mut current_gas = 0;
    let mut ans = 0;
    for i in 0..gas.len() {
        total_gas += gas[i];
        total_cost += cost[i];
        current_gas += gas[i] - cost[i];
        if current_gas < 0 {
            current_gas = 0;
            ans = i + 1;
        }
    }
    if total_gas < total_cost {
        -1
    } else {
        ans as i32
    }
}

// https://leetcode.com/problems/next-permutation/
// https://leetcode.com/problems/next-permutation/editorial/
// Condensed mathematical description:
// 1. Find largest index i such that array[i − 1] < array[i].
// (If no such i exists, then this is already the last permutation.)
// 2. Find largest index j such that j ≥ i and array[j] > array[i − 1]. 3. Swap array[j] and array[i − 1].
// 4. Reverse the suffix starting at array[i].
pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut pivot_idx = -1;
    for i in (1..nums.len()).rev() {
        if nums[i - 1] < nums[i] {
            pivot_idx = (i - 1) as i32;
            break;
        }
    }
    if pivot_idx == -1 {
        nums.reverse();
        return;
    }
    for i in (pivot_idx as usize + 1..nums.len()).rev() {
        if nums[i] >= nums[pivot_idx as usize] {
            nums.swap(i, pivot_idx as usize);
            nums[pivot_idx as usize + 1..].reverse();
            break;
        }
    }
}

// https://leetcode.com/problems/valid-sudoku/
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;
    let mut cache = HashSet::new();
    for i in 0..board.len() {
        for j in 0..board.len() {
            let el = board[i][j];
            if el == '.' {
                continue;
            }
            let row_key = format!("{} in row {}", el, i);
            let column_key = format!("{} in column {}", el, j);
            let sub_square_key = format!("{} in sub-quare {} {}", el, i / 3, j / 3);
            if cache.contains(&row_key)
                || cache.contains(&column_key)
                || cache.contains(&sub_square_key)
            {
                return false;
            }
            cache.insert(row_key);
            cache.insert(column_key);
            cache.insert(sub_square_key);
        }
    }
    true
}

// https://leetcode.com/problems/group-anagrams/
// https://leetcode.com/problems/group-anagrams/editorial/
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    fn build_key(str: &str) -> Vec<i32> {
        let mut key = vec![0; 26];
        for ch in str.chars() {
            key[ch as usize - 'a' as usize] += 1;
        }
        key
    }
    use std::collections::HashMap;
    let mut cache = HashMap::new();
    for word in strs {
        cache
            .entry(build_key(word.as_str()))
            .or_insert(vec![])
            .push(word);
    }
    cache.values().into_iter().cloned().collect()
}

// https://leetcode.com/problems/maximum-product-subarray/
// На ĸаждой итерации мы будем обновлять max_so_far и min_so_far, исходя из трех возможных случаев:
// 1. nums[i] больше или равно нулю: в этом случае мы можем умножить nums[i] на max_so_far и
// получить новое max_so_far, таĸ ĸаĸ умножение на положительное число не меняет знаĸ произведения.
// Мы таĸже можем умножить nums[i] на min_so_far и получить новое min_so_far.
// 2. nums[i] меньше нуля: в этом случае мы можем умножить nums[i] на min_so_far и
// получить новое max_so_far, таĸ ĸаĸ умножение на отрицательное число меняет знаĸ произведения.
// Мы таĸже можем умножить nums[i] на max_so_far и получить новое min_so_far.
// 3. nums[i] равно нулю: в этом случае max_so_far и min_so_far будут равны нулю, таĸ ĸаĸ произведение на ноль дает ноль.
pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut ans = nums[0];
    let mut max_so_far = nums[0];
    let mut min_so_far = nums[0];
    for i in 1..nums.len() {
        let temp_max = (max_so_far * nums[i])
            .max(min_so_far * nums[i])
            .max(nums[i]);
        min_so_far = (min_so_far * nums[i])
            .min(max_so_far * nums[i])
            .min(nums[i]);
        max_so_far = temp_max;
        ans = ans.max(max_so_far);
    }
    ans
}

// https://leetcode.com/problems/design-add-and-search-words-data-structure/
#[derive(Clone, Debug)]
struct TrieNode {
    links: Vec<Option<TrieNode>>,
    is_end: bool,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            links: vec![None; 26],
            is_end: false,
        }
    }
    fn contains(&self, key: char) -> bool {
        self.links[key as usize - 'a' as usize].is_some()
    }
    fn get_mut(&mut self, key: char) -> Option<&mut TrieNode> {
        self.links[key as usize - 'a' as usize].as_mut()
    }
    fn get(&self, key: char) -> Option<&TrieNode> {
        self.links[key as usize - 'a' as usize].as_ref()
    }
    fn put(&mut self, key: char, value: TrieNode) {
        self.links[key as usize - 'a' as usize] = Some(value);
    }
}

struct WordDictionary {
    root: TrieNode,
}
impl WordDictionary {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }
    fn add_word(&mut self, word: String) {
        let mut root = &mut self.root;
        for ch in word.chars() {
            if !root.contains(ch) {
                root.put(ch, TrieNode::new());
            }
            root = root.get_mut(ch).unwrap();
        }
        root.is_end = true;
    }
    fn search(&self, word: String) -> bool {
        self.search_node(word.as_str(), &self.root)
    }
    fn search_node(&self, word: &str, from: &TrieNode) -> bool {
        if word.len() == 1 {
            let ch = word.chars().next().unwrap();
            return if ch == '.' {
                from.links.iter().flatten().filter(|x| x.is_end).count() > 0
            } else {
                from.contains(ch) && from.get(ch).unwrap().is_end
            };
        }
        let mut found = false;
        let ch = word.chars().next().unwrap();
        if ch == '.' {
            for n in from.links.iter().flatten() {
                found |= self.search_node(&word[1..], n)
            }
        } else {
            found |= from.contains(ch) && self.search_node(&word[1..], from.get(ch).unwrap());
        }
        found
    }
}

// https://leetcode.com/problems/pacific-atlantic-water-flow/
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    type Coord = (i32, i32);
    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn dfs(heights: &Vec<Vec<i32>>, coord: Coord, visited: &mut HashSet<Coord>) {
        if visited.contains(&coord) {
            return;
        }
        visited.insert(coord);
        for d in DIR {
            let next_coord = (coord.0 + d.0, coord.1 + d.1);
            if next_coord.0 < 0
                || next_coord.1 < 0
                || next_coord.0 >= heights.len() as i32
                || next_coord.1 >= heights[0].len() as i32
            {
                continue;
            }
            if heights[next_coord.0 as usize][next_coord.1 as usize]
                >= heights[coord.0 as usize][coord.1 as usize]
            {
                dfs(heights, next_coord, visited);
            }
        }
    }

    fn pacific(heights: &Vec<Vec<i32>>) -> HashSet<Coord> {
        let mut visited = HashSet::new();
        for i in 0..heights.len() {
            dfs(heights, (i as i32, 0), &mut visited);
        }
        for i in 0..heights[0].len() {
            dfs(heights, (0, i as i32), &mut visited);
        }
        visited
    }

    fn atlantic(heights: &Vec<Vec<i32>>) -> HashSet<Coord> {
        let mut visited = HashSet::new();
        for i in 0..heights.len() {
            dfs(
                heights,
                (i as i32, heights[0].len() as i32 - 1),
                &mut visited,
            );
        }
        for i in 0..heights[0].len() {
            dfs(heights, (heights.len() as i32 - 1, i as i32), &mut visited);
        }
        visited
    }

    let p_visited = pacific(&heights);
    let a_visited = atlantic(&heights);

    let mut ans = vec![];
    for p in p_visited {
        if a_visited.contains(&p) {
            ans.push(vec![p.0, p.1]);
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_height_trees() {
        println!(
            "{:?}",
            find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            )
        ); // [3,4]
        println!("{:?}", find_min_height_trees(1, vec![])); // [0]
    }

    #[test]
    fn test_least_interval() {
        println!("{}", least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)); // 8
        println!("{}", least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0)); // 6
    }

    #[test]
    fn test_lru_cache() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        println!("{}", lru_cache.get(1)); // 1
        lru_cache.put(3, 3);
        println!("{}", lru_cache.get(2)); // -1
        lru_cache.put(4, 4);
        println!("{}", lru_cache.get(1)); // -1
        println!("{}", lru_cache.get(3)); // 3
        println!("{}", lru_cache.get(4)); // 4

        let mut lru_cache = LRUCache::new(2);
        println!("{}", lru_cache.get(2)); // -1
        lru_cache.put(2, 6);
        println!("{}", lru_cache.get(1)); // -1
        lru_cache.put(1, 5);
        lru_cache.put(1, 2);
        println!("{}", lru_cache.get(1)); // 2
        println!("{}", lru_cache.get(2)); // 6
    }

    #[test]
    fn test_kth_smallest() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", kth_smallest(root, 3)); // 3
    }

    #[test]
    fn test_daily_temperatures() {
        println!(
            "{:?}",
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        ); // [1,1,4,2,1,1,0,0]

        println!("{:?}", daily_temperatures(vec![30, 40, 50, 60])); // [1,1,1,0]
    }

    #[test]
    fn test_rob() {
        println!("{}", rob(vec![1, 2, 3, 1])); // 4
        println!("{}", rob(vec![1, 2])); // 2
        println!("{}", rob(vec![1])); // 1
    }

    #[test]
    fn test_can_complete_circuit() {
        println!(
            "{}",
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        ); // 3
        println!("{}", can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])); // -1
    }

    #[test]
    fn test_next_permutation() {
        let mut v = vec![1, 2, 3];
        next_permutation(&mut v);
        println!("{:?}", v); // [1,3,2]

        let mut v = vec![3, 2, 1];
        next_permutation(&mut v);
        println!("{:?}", v); // [1,2,3]

        let mut v = vec![1, 1, 5];
        next_permutation(&mut v);
        println!("{:?}", v); // [1,5,1]
    }

    #[test]
    fn test_is_valid_sudoku() {
        println!(
            "{}",
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        ); // true

        println!(
            "{}",
            is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        ); // false
    }

    #[test]
    fn test_group_anagrams() {
        println!(
            "{:?}",
            group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string(),
            ])
        ); // [["bat"],["nat","tan"],["ate","eat","tea"]]
    }

    #[test]
    fn test_max_product() {
        println!("{}", max_product(vec![2, 3, -2, 4])); // 6
        println!("{}", max_product(vec![-2, 0, -1])); // 0
        println!("{}", max_product(vec![0, 2])); // 2
        println!("{}", max_product(vec![1, 2, -1, -2, 2, 1, -2, 1, 4, -5, 4])); // 1280
    }

    #[test]
    fn test_search_words() {
        let mut ws = WordDictionary::new();
        ws.add_word("bad".to_string());
        ws.add_word("dad".to_string());
        ws.add_word("mad".to_string());

        println!("{}", ws.search("b..".to_string()));
        println!("{}", ws.search("pad".to_string()));
        println!("{}", ws.search("bad".to_string()));
        println!("{}", ws.search(".ad".to_string()));

        let mut ws = WordDictionary::new();
        ws.add_word("a".to_string());
        ws.add_word("a".to_string());
        println!("{}", ws.search(".".to_string()));
        println!("{}", ws.search("a".to_string()));
        println!("{}", ws.search("aa".to_string()));
        println!("{}", ws.search("a".to_string()));
        println!("{}", ws.search(".a".to_string()));
        println!("{}", ws.search("a.".to_string()));

        let mut ws = WordDictionary::new();
        ws.add_word("at".to_string());
        ws.add_word("and".to_string());
        ws.add_word("an".to_string());
        ws.add_word("add".to_string());

        println!("{}", ws.search("a".to_string()));
        println!("{}", ws.search(".at".to_string()));

        ws.add_word("bat".to_string());

        println!("{}", ws.search(".at".to_string()));
        println!("{}", ws.search("an.".to_string()));
        println!("{}", ws.search("a.d.".to_string()));
        println!("{}", ws.search("d.".to_string()));
        println!("{}", ws.search("d.".to_string()));
        println!("{}", ws.search("a.d".to_string()));
        println!("{}", ws.search(".".to_string()));
    }

    #[test]
    fn test_pacific_atlantic() {
        println!(
            "{:?}",
            pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ])
        ); // [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
    }
}
