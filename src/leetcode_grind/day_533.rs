// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/description/
pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut xor = 0;
    for n in nums {
        xor ^= n;
    }
    (k ^ xor).count_ones() as i32
}

// https://leetcode.com/problems/bitwise-or-of-all-subsequence-sums/submissions/
pub fn subsequence_sum_or(nums: Vec<i32>) -> i64 {
    let mut res = 0;        
    let mut prefix = 0;
    for num in nums {
        prefix += num as i64;
        res |= num as i64 | prefix;
    }
    res
}