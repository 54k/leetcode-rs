// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/description/
pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;
    use std::collections::HashMap;
    let mut depth = HashMap::new();
    depth.insert(-1, -1);

    fn get_depth(node: Node, parent: i32, depth: &mut HashMap<i32, i32>) {
        if node.is_none() {
            return;
        }
        let n = node.clone().unwrap();
        let n = n.borrow();
        depth.insert(n.val, parent + 1);
        get_depth(n.left.clone(), parent + 1, depth);
        get_depth(n.right.clone(), parent + 1, depth);
    }

    fn get_ans(node: Node, depth: &HashMap<i32, i32>, max_depth: i32) -> Node {
        if node.is_none() {
            return None;
        }

        let n = node.clone().unwrap();
        let n = n.borrow();

        if *depth.get(&n.val).unwrap() == max_depth {
            return node.clone();
        }

        let left = get_ans(n.left.clone(), depth, max_depth);
        let right = get_ans(n.right.clone(), depth, max_depth);

        if left.is_some() && right.is_some() {
            return node.clone();
        } else if left.is_some() {
            return left.clone();
        }

        return right;
    }

    get_depth(root.clone(), -1, &mut depth);
    let max_depth = *depth.values().max().unwrap();
    get_ans(root.clone(), &depth, max_depth)
}
