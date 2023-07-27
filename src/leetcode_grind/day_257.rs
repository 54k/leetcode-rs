// https://leetcode.com/problems/maximum-running-time-of-n-computers/description/
pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
    batteries.sort();
    let bn = batteries.len();

    let mut live = batteries
        .iter()
        .copied()
        .skip(bn - n as usize)
        .map(|x| x as i64)
        .collect::<Vec<_>>();

    let mut extra = batteries
        .iter()
        .copied()
        .take(bn - n as usize)
        .map(|x| x as i64)
        .sum::<i64>();

    for i in 0..n as usize - 1 {
        if extra < ((i + 1) as i64) * (live[i + 1] - live[i]) {
            return live[i] + extra / (i as i64 + 1);
        }

        extra -= (i as i64 + 1) * (live[i + 1] - live[i]);
    }

    live[n as usize - 1] + extra / n as i64
}

// https://leetcode.com/problems/symmetric-tree/description/
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back(root.clone());
    q.push_back(root.clone());

    while q.len() > 0 {
        let t1 = q.pop_front().unwrap();
        let t2 = q.pop_front().unwrap();
        if t1.is_none() && t2.is_none() {
            continue;
        }
        if t1.is_none() || t2.is_none() {
            return false;
        }
        let t1 = t1.unwrap().clone();
        let t1 = t1.borrow();
        let t2 = t2.unwrap().clone();
        let t2 = t2.borrow();
        if t1.val != t2.val {
            return false;
        }
        q.push_back(t1.left.clone());
        q.push_back(t2.right.clone());
        q.push_back(t1.right.clone());
        q.push_back(t2.left.clone());
    }

    true
}
