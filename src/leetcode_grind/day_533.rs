// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/description/
pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut xor = 0;
    for n in nums {
        xor ^= n;
    }
    (k ^ xor).count_ones() as i32
}