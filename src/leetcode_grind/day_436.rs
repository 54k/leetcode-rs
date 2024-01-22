// https://leetcode.com/problems/set-mismatch/
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let n = nums.len();
    let mut dup = -1;
    let mut missing = -1;
    let mut map = HashMap::new();
    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }
    for i in 1..=n as i32 {
        if map.contains_key(&i) {
            if map.get(&i).unwrap() == &2 {
                dup = i;
            }
        } else {
            missing = i;
        }
    }
    vec![dup, missing]
}
