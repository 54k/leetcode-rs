use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    const MOD: i64 = 1000000007;
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_product: &mut i64, total_sum: &mut i64) -> i64 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let root = root.borrow();

        let left = dfs(root.left.clone(), max_product, total_sum);
        let right = dfs(root.right.clone(), max_product, total_sum);
        let sum = left + root.val as i64 + right;
        *max_product = (*max_product).max(sum * (*total_sum - sum));
        sum
    }

    let mut max_product = i64::MIN;
    let mut total_sum = 0;
    total_sum = dfs(root.clone(), &mut max_product, &mut total_sum);
    dfs(root, &mut max_product, &mut total_sum);
    ((max_product % MOD + MOD) % MOD) as i32
}
