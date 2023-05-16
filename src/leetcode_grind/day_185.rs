// https://leetcode.com/problems/find-the-k-beauty-of-a-number/description/
pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    todo!()
}

// https://leetcode.com/problems/recover-binary-search-tree/description/
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

// https://leetcode.com/problems/delete-node-in-a-bst/description/
pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    todo!()
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
