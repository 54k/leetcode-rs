// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/description
pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    nums.sort();

    for n in nums {
        let len = res.len() - 1;
        if res.is_empty() || res[len].len() == 3 {
            res.push(vec![n]);
        } else if n - res[len][0] <= k {
            res[len].push(n);
        } else {
            return vec![];
        }
    }

    res
}
