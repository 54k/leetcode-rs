// https://leetcode.com/problems/bitwise-xor-of-all-pairings/description/?envType=daily-question&envId=2025-01-16
pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut xor1 = 0;
    let mut xor2 = 0;
    if nums1.len() % 2 == 1 {
        for &num in &nums2 {
            xor2 ^= num;
        }
    }
    if nums2.len() % 2 == 1 {
        for &num in &nums1 {
            xor1 ^= num;
        }
    }
    xor1 ^ xor2
}
