use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, go_left: bool, steps: i32, longest_path: &mut i32) {
        if let Some(r) = root {
            *longest_path = (*longest_path).max(steps);
            if go_left {
                dfs(r.borrow().left.clone(), false, steps + 1, longest_path);
                dfs(r.borrow().right.clone(), true, 1, longest_path);
            } else {
                dfs(r.borrow().right.clone(), true, steps + 1, longest_path);
                dfs(r.borrow().left.clone(), false, 1, longest_path);
            }
        }
    }
    let mut longest_path = 0;
    dfs(root.clone(), true, 0, &mut longest_path);
    dfs(root, false, 0, &mut longest_path);
    longest_path
}

// https://leetcode.com/problems/reduce-array-size-to-the-half/description/
pub fn min_set_size(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = arr.len() as i32;
    let mut map = HashMap::new();
    for num in arr {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut ans = 0;
    let mut freqs = map.values().copied().collect::<Vec<i32>>();
    freqs.sort();
    let mut removed = 0;
    for i in (0..freqs.len()).rev() {
        removed += freqs[i];
        ans += 1;
        if removed >= n / 2 {
            break;
        }
    }
    ans
}

// https://leetcode.com/problems/how-many-apples-can-you-put-into-the-basket/description/
pub fn max_number_of_apples(weight: Vec<i32>) -> i32 {
    let max = weight.iter().copied().max().unwrap();
    let mut buckets = vec![0; max as usize + 1];
    for w in weight {
        buckets[w as usize] += 1;
    }
    let mut w = 0;
    let mut count = 0;
    for i in 0..max as usize + 1 {
        if buckets[i] > 0 {
            let c = buckets[i].min((5000 - w) / i);
            if c == 0 {
                break;
            }
            w += c * i;
            count += c;
        }
    }
    count as i32
}
