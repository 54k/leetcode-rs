// https://leetcode.com/problems/binary-subarrays-with-sum/description
pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    fn sliding_window_at_most(nums: &Vec<i32>, goal: i32) -> i32 {
        let mut start = 0;
        let mut current_sum = 0;
        let mut total_count = 0;

        for end in 0..nums.len() {
            current_sum += nums[end];

            while start <= end && current_sum > goal {
                current_sum -= nums[start];
                start += 1;
            }

            total_count += end - start + 1;
        }
        total_count as i32
    }

    sliding_window_at_most(&nums, goal) - sliding_window_at_most(&nums, goal - 1)
}
