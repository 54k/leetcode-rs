// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/description/
pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    for k in 1..=3 {
        for i in 0..nums.len() - k {
            if nums[i] == nums[i + k] {
                return nums[i];
            }
        }
    }
    panic!();
}
