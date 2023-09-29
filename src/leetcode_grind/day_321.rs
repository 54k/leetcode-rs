// https://leetcode.com/problems/monotonic-array/description
pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut prev = 0;
    for i in 0..nums.len() - 1 {
        let comp = if nums[i] < nums[i + 1] {
            -1
        } else if nums[i] == nums[i + 1] {
            0
        } else {
            1
        };

        if comp != 0 && prev != 0 && comp != prev {
            return false;
        }
        if comp != 0 {
            prev = comp;
        }
    }
    true
}
