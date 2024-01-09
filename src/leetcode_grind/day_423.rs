// https://leetcode.com/problems/check-if-an-array-is-consecutive/description/
pub fn is_consecutive(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let len = nums.len();
    let mut set = HashSet::new();
    for num in nums {
        set.insert(num);
    }

    let n = set.len();
    if n != len {
        return false;
    }

    for &num in &set {
        if !set.contains(&(num - 1)) {
            let mut streak = 1;
            let mut curr_num = num;
            while set.contains(&(curr_num + 1)) {
                streak += 1;
                curr_num += 1;
            }
            if streak == n {
                return true;
            }
        }
    }
    false
}
