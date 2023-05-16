use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/find-the-k-beauty-of-a-number/description/
// https://leetcode.com/problems/find-the-k-beauty-of-a-number/solutions/2746011/rust-solution-0-ms/
pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    let mut ans = 0;
    let num_s = format!("{}", num).chars().collect::<Vec<char>>();
    let mut s = 0;
    for e in 0..num_s.len() {
        if e >= k as usize - 1 {
            let c = i32::from_str_radix(
                num_s[s..=e].iter().copied().collect::<String>().as_str(),
                10,
            )
            .unwrap();
            if num % c == 0 {
                ans += 1;
            }
            s += 1;
        }
    }
    ans
}

// https://leetcode.com/problems/recover-binary-search-tree/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn find_two_swapped(nums: &Vec<i32>) -> (i32, i32) {
        let n = nums.len();
        let (mut x, mut y) = (-1, -1);
        let mut swapped_first_occurence = false;
        for i in 0..n - 1 {
            if nums[i + 1] < nums[i] {
                y = nums[i + 1];
                if !swapped_first_occurence {
                    x = nums[i];
                    swapped_first_occurence = true;
                } else {
                    // second swap occurence
                    break;
                }
            }
        }
        return (x, y);
    }

    fn inorder(root: &mut Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(r) = root {
            let r = r.borrow();
            inorder(&mut r.left.clone(), nums);
            nums.push(r.val);
            inorder(&mut r.right.clone(), nums);
        }
    }

    fn recover(root: &mut Option<Rc<RefCell<TreeNode>>>, mut cnt: i32, (x, y): (i32, i32)) {
        if let Some(mut r) = root.clone() {
            let mut r = r.borrow_mut();
            if r.val == x || r.val == y {
                r.val = if r.val == x { y } else { x };
                cnt -= 1;
                if cnt == 0 {
                    return;
                }
            }
            recover(&mut r.left.clone(), cnt, (x, y));
            recover(&mut r.right.clone(), cnt, (x, y));
        }
    }

    let mut nums = vec![];
    inorder(&mut root.clone(), &mut nums);
    let swapped = find_two_swapped(&nums);
    recover(&mut root.clone(), 2, swapped);
}

pub fn recover_tree2(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn find_two_swapped(
        root: Option<Rc<RefCell<TreeNode>>>,
        pred: &mut Option<Rc<RefCell<TreeNode>>>,
        x: &mut Option<Rc<RefCell<TreeNode>>>,
        y: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(r) = root.clone() {
            let r = r.borrow();
            find_two_swapped(r.left.clone(), pred, x, y);

            if pred.is_some() && pred.as_ref().unwrap().borrow().val > r.val {
                *y = root.clone();
                if x.is_none() {
                    *x = pred.clone();
                } else {
                    return;
                }
            }
            *pred = root.clone();

            find_two_swapped(r.right.clone(), pred, x, y);
        }
    }
    let (mut x, mut y) = (None, None);
    find_two_swapped(root.clone(), &mut None, &mut x, &mut y);
    std::mem::swap(
        &mut x.unwrap().borrow_mut().val,
        &mut y.unwrap().borrow_mut().val,
    );
}

// https://leetcode.com/problems/gray-code/description/
pub fn gray_code(n: i32) -> Vec<i32> {
    let mut ans = vec![];
    ans.push(0);
    for i in 1..=n {
        let prev_len = ans.len();
        let mask = 1 << (i - 1);
        for j in (0..prev_len).rev() {
            ans.push(mask + ans[j])
        }
    }
    ans
}

// https://leetcode.com/problems/permutation-sequence/
pub fn get_permutation(n: i32, mut k: i32) -> String {
    let mut factorials = vec![0; n as usize];
    factorials[0] = 1;
    let mut nums = vec![1];

    for i in 1..n as usize {
        factorials[i] = factorials[i - 1] * i as i32;
        nums.push(i as i32 + 1);
    }

    k -= 1;
    let mut ans = String::new();
    for i in (0..n as usize).rev() {
        let mut idx = k / factorials[i];
        k -= idx * factorials[i];
        ans.push_str(&format!("{}", nums[idx as usize]));
        nums.remove(idx as usize);
    }
    ans
}
