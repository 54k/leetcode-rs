// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/description/?envType=daily-question&envId=2024-08-02
pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let mut minimum_swaps = i32::MAX;
    let mut total_ones = 0;
    for &num in &nums {
        total_ones += num;
    }

    let mut ones_count = nums[0];
    let mut end = 0;

    for start in 0..nums.len() {
        if (start != 0) {
            ones_count -= nums[start - 1];
        }
        while end - start + 1 < total_ones as usize {
            end += 1;
            ones_count += nums[end % nums.len()];
        }
        minimum_swaps = minimum_swaps.min(total_ones - ones_count);
    }

    minimum_swaps
}
