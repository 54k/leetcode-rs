// https://leetcode.com/problems/find-the-k-beauty-of-a-number/description/
pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    todo!()
}

// https://leetcode.com/problems/recover-binary-search-tree/description/
pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
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
