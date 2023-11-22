// https://leetcode.com/problems/diagonal-traverse-ii/
pub fn find_diagonal_order_1(nums: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut diag = HashMap::new();

    for r in (0..nums.len()).rev() {
        for c in 0..nums[r].len() {
            let d = r + c;
            diag.entry(d).or_insert(vec![]).push(nums[r][c]);
        }
    }

    let mut ans = vec![];
    let mut d = 0;
    while (diag.contains_key(&d)) {
        ans.extend(diag.remove(&d).unwrap().into_iter());
        d += 1;
    }
    ans
}

pub fn find_diagonal_order_2(nums: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut ans = vec![];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while !queue.is_empty() {
        let (r, c) = queue.pop_front().unwrap();
        ans.push(nums[r][c]);

        if c == 0 && r + 1 < nums.len() {
            queue.push_back((r + 1, c));
        }

        if c + 1 < nums[r].len() {
            queue.push_back((r, c + 1));
        }
    }
    ans
}
