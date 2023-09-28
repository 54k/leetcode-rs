// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/description/
pub fn find_maximum_xor_hashset(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut max_len = 0;
    let mut max_num = *nums.iter().max().unwrap();
    while max_num > 0 {
        max_num >>= 1;
        max_len += 1;
    }

    let mut max_xor = 0;
    let mut prefixes = HashSet::new();
    for i in (0..max_len).rev() {
        max_xor <<= 1;
        let cur_xor = max_xor | 1;
        prefixes.clear();

        for &num in &nums {
            prefixes.insert(num >> i);
        }

        for &p in &prefixes {
            if prefixes.contains(&(cur_xor ^ p)) {
                max_xor = cur_xor;
                break;
            }
        }
    }
    max_xor
}
