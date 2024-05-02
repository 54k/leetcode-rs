// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/description
pub fn find_max_k(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut mx = i32::MIN;
    let mut hash = HashSet::new();
    for num in nums {
        hash.insert(num);
        if hash.contains(&(-num)) {
            mx = mx.max(num.abs());
        }
    }
    if mx == i32::MIN {
        return -1;
    }
    mx
}
