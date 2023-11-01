// https://leetcode.com/problems/diagonal-traverse-ii/description/
pub fn find_diagonal_order_tle(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let mut res_ptr = 0usize;

    let m = nums.len();
    let mut n = 0;
    for i in 0..m {
        n = n.max(nums[m - 1].len());
    }

    for r in 0..m + n - 1 {
        let mut i = if r < m { r } else { m - 1 } as i32;
        let mut j = if r < m { 0 } else { r - m + 1 } as i32;

        while i >= 0 {
            if nums[i as usize].len() > j as usize {
                result.push(nums[i as usize][j as usize]);
                i -= 1;
                j += 1;
            }
        }
        // println!("{} {}", i, j);
    }

    // println!("{:?}", result);
    result
}

pub fn find_diagonal_order_hash_map(nums: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut diagonals = HashMap::new();
    for i in (0..nums.len()).rev() {
        for j in 0..nums[i].len() {
            diagonals.entry(i + j).or_insert(vec![]).push(nums[i][j]);
        }
    }
    let mut ans = vec![];
    let mut curr_diag = 0;
    while diagonals.contains_key(&curr_diag) {
        ans.extend(diagonals.remove(&curr_diag).unwrap().iter());
        curr_diag += 1;
    }
    ans
}