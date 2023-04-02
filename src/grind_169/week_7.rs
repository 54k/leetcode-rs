// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn preorder_serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
            if let Some(r) = root {
                format!(
                    "{},{},{}",
                    r.as_ref().borrow().val,
                    preorder_serialize(r.as_ref().borrow().left.clone()),
                    preorder_serialize(r.as_ref().borrow().right.clone()),
                )
            } else {
                "X".to_string()
            }
        }
        preorder_serialize(root)
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn preorder_deserialize(data: &Vec<&str>, i: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
            *i += 1;
            let val = data[*i as usize];
            if val == "X" {
                return None;
            }
            let root = TreeNode {
                val: val.parse().unwrap(),
                left: preorder_deserialize(data, i),
                right: preorder_deserialize(data, i),
            };
            Some(Rc::new(RefCell::new(root)))
        }
        preorder_deserialize(&data.split(',').collect(), &mut -1)
    }
}

// https://leetcode.com/problems/trapping-rain-water/
// https://leetcode.com/problems/trapping-rain-water/editorial/
pub fn trap(height: Vec<i32>) -> i32 {
    fn brute_force_approach(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let size = height.len();
        for i in 1..size - 1 {
            let mut left_max = 0;
            let mut right_max = 0;
            for j in (0..=i).rev() {
                left_max = left_max.max(height[j]);
            }
            for j in i..size {
                right_max = right_max.max(height[j]);
            }
            ans += left_max.min(right_max) - height[i];
        }
        ans
    }
    fn dynamic_programming_approach(height: Vec<i32>) -> i32 {
        let mut left_max = vec![0; height.len()];
        left_max[0] = height[0];
        for i in 1..height.len() {
            left_max[i] = height[i].max(left_max[i - 1]);
        }

        let mut right_max = vec![0; height.len()];
        right_max[height.len() - 1] = height[height.len() - 1];
        for i in (0..height.len() - 1).rev() {
            right_max[i] = height[i].max(right_max[i + 1]);
        }

        let mut ans = 0;
        for i in 0..height.len() {
            ans += left_max[i].min(right_max[i]) - height[i];
        }
        ans
    }
    fn stacks_approach(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut current = 0;
        let mut stack = vec![];
        while current < height.len() {
            while !stack.is_empty() && height[*stack.last().unwrap()] <= height[current] {
                let top = stack.pop().unwrap();
                if stack.is_empty() {
                    break;
                }
                let distance = current - *stack.last().unwrap() - 1;
                let bounded_height =
                    height[current].min(height[*stack.last().unwrap()]) - height[top];
                ans += distance as i32 * bounded_height;
            }
            stack.push(current);
            current += 1;
        }
        ans
    }
    fn two_pointers_approach(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut ans = 0;
        let mut left_max = 0;
        let mut right_max = 0;
        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    ans += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    ans += right_max - height[right];
                }
                right -= 1;
            }
        }
        ans
    }
    two_pointers_approach(height)
}

// https://leetcode.com/problems/find-median-from-data-stream/
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }
    fn add_num(&mut self, num: i32) {
        if self.left.is_empty() || *self.left.peek().unwrap() > num {
            self.left.push(num);
        } else {
            self.right.push(Reverse(num));
        }
        if self.left.len() > self.right.len() + 1 {
            self.right.push(Reverse(self.left.pop().unwrap()));
        }
        if self.right.len() > self.left.len() {
            self.left.push(self.right.pop().unwrap().0);
        }
    }
    fn find_median(&self) -> f64 {
        if (self.left.len() + self.right.len()) % 2 == 1 {
            *self.left.peek().unwrap() as f64
        } else {
            (*self.left.peek().unwrap() + self.right.peek().unwrap().0) as f64 / 2.0
        }
    }
}

// https://leetcode.com/problems/word-ladder/description/
// https://leetcode.com/problems/word-ladder/editorial/
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    fn bfs_approach(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet, VecDeque};
        let w_len = begin_word.len();
        let mut adj = HashMap::new();
        for word in word_list {
            for i in 0..w_len {
                let new_word = format!("{}*{}", &word[0..i], &word[i + 1..]);
                adj.entry(new_word)
                    .or_insert(vec![])
                    .push(word[0..].to_string());
            }
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((begin_word, 1));

        while let Some((word, level)) = queue.pop_front() {
            visited.insert(word.clone());
            for i in 0..w_len {
                let lookup_key = format!("{}*{}", &word[0..i], &word[i + 1..]);
                let empty = vec![];
                let n_list = adj.get(&lookup_key).unwrap_or(&empty);
                for neighbor in n_list {
                    if neighbor == &end_word {
                        return level + 1;
                    }
                    if !visited.contains(neighbor) {
                        queue.push_back((neighbor.clone(), level + 1));
                    }
                }
            }
        }
        0
    }
    fn bidi_search_bfs_approach(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> i32 {
        use std::collections::{HashMap, VecDeque};
        fn visit_node(
            queue: &mut VecDeque<(String, i32)>,
            visited: &mut HashMap<String, i32>,
            visited_other: &mut HashMap<String, i32>,
            adj: &HashMap<String, Vec<String>>,
        ) -> i32 {
            for _ in 0..queue.len() {
                let (word, level) = queue.pop_front().unwrap();
                for i in 0..word.len() {
                    let lookup_key = format!("{}*{}", &word[0..i], &word[i + 1..]);
                    let empty_vec = vec![];
                    let n_vec = adj.get(&lookup_key).unwrap_or(&empty_vec);
                    for neighbor in n_vec {
                        if visited_other.contains_key(neighbor) {
                            return level + visited_other[neighbor];
                        }

                        if !visited.contains_key(neighbor) {
                            visited.insert(neighbor.clone(), level + 1);
                            queue.push_back((neighbor.clone(), level + 1));
                        }
                    }
                }
            }
            -1
        }
        if !word_list.contains(&end_word) {
            return 0;
        }
        let w_len = begin_word.len();
        let mut adj = HashMap::new();
        for word in word_list {
            for i in 0..w_len {
                let new_word = format!("{}*{}", &word[0..i], &word[i + 1..]);
                adj.entry(new_word)
                    .or_insert(vec![])
                    .push(word[0..].to_string());
            }
        }

        let mut visit_begin = HashMap::new();
        visit_begin.insert(begin_word.clone(), 1);
        let mut visit_end = HashMap::new();
        visit_end.insert(end_word.clone(), 1);

        let mut start_queue = VecDeque::new();
        start_queue.push_back((begin_word, 1));
        let mut end_queue = VecDeque::new();
        end_queue.push_back((end_word, 1));

        while !start_queue.is_empty() && !end_queue.is_empty() {
            let ans = if start_queue.len() <= end_queue.len() {
                visit_node(&mut start_queue, &mut visit_begin, &mut visit_end, &adj)
            } else {
                visit_node(&mut end_queue, &mut visit_end, &mut visit_begin, &adj)
            };
            if ans > -1 {
                return ans;
            }
        }
        0
    }
    bidi_search_bfs_approach(begin_word, end_word, word_list)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_codec() {
        // it works
    }

    #[test]
    fn test_trap() {
        println!("{}", trap(vec![4, 2, 0, 3, 2, 5])); // 9
        println!("{}", trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
    }

    #[test]
    fn test_median_finder() {
        let mut mf = MedianFinder::new();
        mf.add_num(4);
        mf.add_num(6);
        mf.add_num(5);
        println!("{}", mf.find_median()); // 5
    }

    #[test]
    fn test_ladder_length() {
        println!(
            "{}",
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string()
                ]
            )
        );

        println!(
            "{}",
            ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string()
                ]
            )
        );

        println!(
            "{}",
            ladder_length(
                "a".to_string(),
                "c".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string()]
            )
        );
    }
}
