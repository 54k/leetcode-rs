// https://leetcode.com/problems/build-array-from-permutation/description/?envType=daily-question&envId=2025-05-06
pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();        
    for i in 0..n {
        nums[i] += 1000 * (nums[nums[i] as usize] % 1000);
    }
    for i in 0..n {
        nums[i] /= 1000;
    }
    nums
}