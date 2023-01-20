// https://leetcode.com/problems/non-decreasing-subsequences/solutions/2910678/increasing-subsequences/?orderBy=most_relevant
pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::*;
    fn backtrack(nums: &[i32], i: usize, cur: &mut Vec<i32>, res: &mut HashSet<Vec<i32>>) {
        if i >= nums.len() {
            if cur.len() >= 2 {
                res.insert(cur.clone());
            }
            return;
        }
        if cur.is_empty() || *cur.last().unwrap() <= nums[i] {
            cur.push(nums[i]);
            backtrack(nums, i + 1, cur, res);
            cur.pop();
        }
        backtrack(nums, i + 1, cur, res);
    }
    let mut res = HashSet::new();
    backtrack(&nums, 0, &mut vec![], &mut res);
    res.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test162() {
        println!("{:?}", find_subsequences(vec![4, 6, 7, 7])); // [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
    }
}
