// https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/description/?envType=daily-question&envId=2024-11-16
pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 1 {
        return nums;
    }

    let length = nums.len();
    let mut result = vec![-1; length - k as usize + 1];

    let mut consecutive_count = 1;

    for index in 0..length - 1 {
        if (nums[index] + 1 == nums[index + 1]) {
            consecutive_count += 1;
        } else {
            consecutive_count = 1;
        }

        if consecutive_count >= k {
            result[index - k as usize + 2] = nums[index + 1];
        }
    }

    result
}
