// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/description
pub fn min_operations(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for n in nums {
        *freq.entry(n).or_insert(0) += 1;
    }

    let mut ans = 0;
    for &v in freq.values() {
        if v == 1 {
            return -1;
        }
        ans += ((v as f64) / 3.0).ceil() as i32;
    }
    ans
}
