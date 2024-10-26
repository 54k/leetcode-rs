#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

static mut max_height_after_removal: [i32; 100001] = [0; 100001];
static mut curr_max_height: i32 = 0i32;

use std::cell::RefCell;
use std::rc::Rc;

pub fn tree_queries_i(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn left_to_right(root: Node, curr_height: i32) {
        if let Some(r) = root {
            unsafe {
                max_height_after_removal[r.borrow().val as usize] = curr_max_height;
                curr_max_height = curr_max_height.max(curr_height);
                left_to_right(r.borrow().left.clone(), curr_height + 1);
                left_to_right(r.borrow().right.clone(), curr_height + 1);
            }
        }
    }
    fn right_to_left(root: Node, curr_height: i32) {
        if let Some(r) = root {
            unsafe {
                max_height_after_removal[r.borrow().val as usize] =
                    max_height_after_removal[r.borrow().val as usize].max(curr_max_height);
                curr_max_height = curr_max_height.max(curr_height);
                right_to_left(r.borrow().right.clone(), curr_height + 1);
                right_to_left(r.borrow().left.clone(), curr_height + 1);
            }
        }
    }

    let mut qcount = queries.len();
    let mut qresult = vec![0; qcount];
    unsafe {
        curr_max_height = 0;
        left_to_right(root.clone(), 0);
        curr_max_height = 0;
        right_to_left(root.clone(), 0);
        for i in 0..qcount {
            qresult[i] = max_height_after_removal[queries[i] as usize];
        }
    }
    qresult
}

pub fn tree_queries_ii(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn height(node: Node, height_cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(n) = node {
            if height_cache.contains_key(&n.borrow().val) {
                return height_cache[&n.borrow().val];
            }

            let h = 1 + height(n.borrow().left.clone(), height_cache)
                .max(height(n.borrow().right.clone(), height_cache));
            height_cache.insert(n.borrow().val, h);
            return h;
        }
        -1
    }

    fn dfs(
        node: Node,
        depth: i32,
        max_val: i32,
        result_map: &mut HashMap<i32, i32>,
        height_cache: &mut HashMap<i32, i32>,
    ) {
        if let Some(n) = node {
            result_map.insert(n.borrow().val, max_val);
            dfs(
                n.borrow().left.clone(),
                depth + 1,
                max_val.max(depth + 1 + height(n.borrow().right.clone(), height_cache)),
                result_map,
                height_cache,
            );
            dfs(
                n.borrow().right.clone(),
                depth + 1,
                max_val.max(depth + 1 + height(n.borrow().left.clone(), height_cache)),
                result_map,
                height_cache,
            );
        }
    }
    let mut result_map = HashMap::new();
    let mut height_cache = HashMap::new();
    let mut qcount = queries.len();
    let mut qresult = vec![0; qcount];
    dfs(root.clone(), 0, 0, &mut result_map, &mut height_cache);
    for i in 0..qcount {
        qresult[i] = result_map[&queries[i]];
    }
    qresult
}
