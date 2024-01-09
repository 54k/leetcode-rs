// https://leetcode.com/problems/rotate-array/description/
pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
    let mut a = vec![0; nums.len()];
    for i in 0..nums.len() {
        a[(i + k as usize) % nums.len()] = nums[i];
    }
    for i in 0..nums.len() {
        nums[i] = a[i];
    }
}
