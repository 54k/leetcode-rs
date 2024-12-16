// https://leetcode.com/problems/continuous-subarrays/description/
pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
    let mut left = 0;
    let mut right = 0;

    let mut cur_min = nums[right];
    let mut cur_max = nums[right];

    let mut window_len = 0;
    let mut total = 0;

    while right < nums.len() {
        cur_min = cur_min.min(nums[right]);
        cur_max = cur_max.max(nums[right]);

        if cur_max - cur_min > 2 {
            window_len = right - left;
            total += (window_len as i64) * (window_len as i64 + 1) / 2;

            left = right;
            cur_min = nums[right];
            cur_max = nums[right];

            while left > 0 && (nums[right] - nums[left - 1]).abs() <= 2 {
                left -= 1;
                cur_min = cur_min.min(nums[left]);
                cur_max = cur_max.max(nums[left]);
            }

            if left < right {
                window_len = right - left;
                total -= (window_len as i64) * (window_len as i64 + 1) / 2;
            }
        }

        right += 1;
    }

    window_len = right - left;
    total += (window_len as i64) * (window_len as i64 + 1) / 2;
    total
}
