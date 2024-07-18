// https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/description/
pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut unique_positive_nums = HashSet::new();
    for num in nums {
        if num > 0 {
            unique_positive_nums.insert(num);
        }
    }
    unique_positive_nums.len() as i32
}
