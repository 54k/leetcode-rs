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

use std::cell::RefCell;
// https://leetcode.com/problems/lru-cache/description/
use std::collections::HashMap;
use std::ptr::null_mut;
use std::rc::Rc;

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

// House robber
// The non-recursive solution to the house robber problem uses an iterative approach to solve the problem, rather than a recursive approach.
// Instead of calling a function recursively to solve the problem, it uses a loop to iterate over the input data and update the solution as it goes.
// The key to this solution is the use of two variables, prev and last, to store the maximum amount of money that can be robbed so far. At each iteration of the loop, the solution updates these variables based on the current value of curr, which represents the value of the current house.
// The solution uses the following logic:
// If the current house is not robbed, then last is set to the maximum of prev (the maximum amount of money that can be robbed so far, excluding the current house) and last (the maximum amount of money that can be robbed so far, including the previous house but not the current one).
// If the current house is robbed, then last is set to prev + curr, which represents the maximum amount of money that can be robbed so far, including the current house.
// This approach allows the solution to compute the maximum amount of money that can be robbed so far in a single pass through the input data, without the need for recursive function calls.
// The recursive solution, on the other hand, uses a recursive function to compute the maximum amount of money that can be robbed so far by calling itself with different combinations of the input data and intermediate results. This can be less efficient than an iterative approach, especially for larger inputs, because it requires a function call for each element in the input data.

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
}
