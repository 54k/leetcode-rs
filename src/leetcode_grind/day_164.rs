// https://leetcode.com/problems/house-robber-ii/
pub fn rob(nums: Vec<i32>) -> i32 {
    fn rob_simple(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let (mut t1, mut t2) = (0, 0);
        for i in start..=end {
            let temp = t1;
            let current = nums[i];
            t1 = (current + t2).max(t1);
            t2 = temp;
        }
        t1
    }
    if nums.is_empty() {
        0
    } else if nums.len() == 1 {
        nums[0]
    } else {
        rob_simple(&nums, 0, nums.len() - 2).max(rob_simple(&nums, 1, nums.len() - 1))
    }
}

// https://leetcode.com/problems/smallest-number-in-infinite-set/
mod sis_my_solution {
    use std::collections::BinaryHeap;

    pub struct SmallestInfiniteSet {
        min_heap: BinaryHeap<i32>,
        set: Vec<bool>,
    }

    impl SmallestInfiniteSet {
        pub fn new() -> Self {
            Self {
                min_heap: BinaryHeap::from(vec![-1]),
                set: vec![true; 1001],
            }
        }

        pub fn pop_smallest(&mut self) -> i32 {
            let mut pop = self.min_heap.pop().unwrap();
            let ans = -pop;
            self.set[ans as usize] = false;
            while !self.set[-pop as usize + 1] {
                pop -= 1;
            }
            if !self.min_heap.is_empty() && *self.min_heap.peek().unwrap() == pop - 1 {
                return ans;
            }
            self.min_heap.push(pop - 1);
            ans
        }

        pub fn add_back(&mut self, num: i32) {
            if !self.set[num as usize] {
                self.set[num as usize] = true;
                if -(*self.min_heap.peek().unwrap_or(&(-num))) > num {
                    self.min_heap.push(-num);
                }
            }
        }
    }
}

mod sis_leetcode_solution {
    use std::collections::BinaryHeap;
    use std::collections::HashSet;
    pub struct SmallestInfiniteSet {
        heap: BinaryHeap<i32>,
        set: HashSet<i32>,
        current: i32,
    }
    impl SmallestInfiniteSet {
        pub fn new() -> Self {
            Self {
                heap: BinaryHeap::new(),
                set: HashSet::new(),
                current: 1,
            }
        }

        pub fn pop_smallest(&mut self) -> i32 {
            if !self.heap.is_empty() {
                let pop = -self.heap.pop().unwrap();
                self.set.remove(&pop);
                pop
            } else {
                let ans = self.current;
                self.current += 1;
                ans
            }
        }

        pub fn add_back(&mut self, num: i32) {
            if self.current <= num || self.set.contains(&num) {
                return;
            }
            self.set.insert(num);
            self.heap.push(-num);
        }
    }
}

// https://leetcode.com/problems/search-suggestions-system/description/
pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    #[derive(Clone)]
    struct TrieNode {
        children: Vec<Option<TrieNode>>,
        is_end: bool,
    }
    impl TrieNode {
        fn new() -> Self {
            Self {
                children: vec![None; 26],
                is_end: false,
            }
        }
        fn insert(&mut self, ch: char) {
            self.children[ch as usize - 'a' as usize] = Some(TrieNode::new());
        }
        fn get(&self, ch: char) -> Option<&TrieNode> {
            self.children[ch as usize - 'a' as usize].as_ref()
        }
        fn get_mut(&mut self, ch: char) -> Option<&mut TrieNode> {
            self.children[ch as usize - 'a' as usize].as_mut()
        }
    }
    struct Trie {
        root: TrieNode,
    }
    impl Trie {
        fn new() -> Self {
            Trie {
                root: TrieNode::new(),
            }
        }
        fn insert(&mut self, word: String) {
            let mut node = &mut self.root;
            for ch in word.chars() {
                if node.get_mut(ch).is_none() {
                    node.insert(ch);
                }
                node = node.get_mut(ch).unwrap();
            }
            node.is_end = true;
        }
        fn search(&self, prefix: String) -> Vec<String> {
            let mut result = vec![];
            let mut node = &self.root;
            for ch in prefix.chars() {
                if node.get(ch).is_none() {
                    return result;
                }
                node = node.get(ch).unwrap();
            }
            self.dfs(node, prefix, &mut result);
            result
        }
        fn dfs(&self, node: &TrieNode, prefix: String, result: &mut Vec<String>) {
            if result.len() == 3 {
                return;
            }
            if node.is_end {
                result.push(prefix.clone());
            }
            for ch in 'a'..='z' {
                if let Some(child) = node.get(ch) {
                    self.dfs(child, format!("{}{}", prefix, ch), result);
                }
            }
        }
    }

    let mut trie = Trie::new();
    for product in products {
        trie.insert(product);
    }

    let mut ans = vec![];
    let mut prefix = String::new();
    for ch in search_word.chars() {
        prefix.push(ch);
        ans.push(trie.search(prefix.clone()));
    }
    ans
}

// https://leetcode.com/problems/hamming-distance/description/
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut dist = 0;
    for i in 0..31 {
        if x >> i & 1 != y >> i & 1 {
            dist += 1;
        }
    }
    dist
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test465() {
        println!("{}", rob(vec![1, 2, 3, 1])); // 4
    }

    #[test]
    fn test466() {
        // ["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
        // [[], [2], [], [], [], [1], [], [], []]
        // Output: [null, null, 1, 2, 3, null, 1, 4, 5]

        let mut sis = sis_my_solution::SmallestInfiniteSet::new();
        sis.add_back(2);
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        sis.add_back(1);
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());

        let mut sis = sis_leetcode_solution::SmallestInfiniteSet::new();
        sis.add_back(2);
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        sis.add_back(1);
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
        println!("{}", sis.pop_smallest());
    }

    #[test]
    fn test447() {
        println!(
            "{:?}",
            suggested_products(
                vec![
                    "mobile".to_string(),
                    "mouse".to_string(),
                    "moneypot".to_string(),
                    "monitor".to_string(),
                    "mousepad".to_string()
                ],
                "mouse".to_string()
            )
        );
    }

    #[test]
    fn test448() {
        println!("{}", hamming_distance(1, 4)); // 2
        println!("{}", hamming_distance(1, 3)); // 1
    }
}
