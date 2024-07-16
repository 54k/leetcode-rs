// https://leetcode.com/problems/find-distance-in-a-binary-tree/description/?envType=weekly-question&envId=2024-07-15
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

type NODE = Option<Rc<RefCell<TreeNode>>>;
use std::cell::RefCell;
use std::rc::Rc;
pub fn find_distance(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> i32 {
    fn distance(root: NODE, p: i32, q: i32, depth: i32) -> i32 {
        if root.is_none() || p == q {
            return 0;
        }

        let r = root.unwrap();
        let r = r.borrow();

        if r.val == p || r.val == q {
            let left = distance(r.left.clone(), p, q, 1);
            let right = distance(r.right.clone(), p, q, 1);
            return if left > 0 || right > 0 {
                left.max(right)
            } else {
                depth
            };
        }
        let left = distance(r.left.clone(), p, q, depth + 1);
        let right = distance(r.right.clone(), p, q, depth + 1);
        let mut ret_distance = left + right;

        if left != 0 && right != 0 {
            ret_distance -= 2 * depth;
        }
        ret_distance
    }
    distance(root, p, q, 0)
}

// https://leetcode.com/problems/car-fleet/description/
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars = position.iter().zip(speed).collect::<Vec<_>>();
    cars.sort_by(|a, b| b.cmp(a));
    let mut stack = vec![];
    for (pos, spe) in cars {
        let rea = (target - pos) as f64 / spe as f64;
        if stack.len() < 1 || stack[stack.len() - 1] < rea {
            stack.push(rea);
        }
    }
    stack.len() as i32
}
