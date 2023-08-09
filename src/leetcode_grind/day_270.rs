// https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/description/
pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
    fn is_valid(nums: &Vec<i32>, p: i32, threshold: i32) -> bool {
        let mut index = 0;
        let mut count = 0;
        while index < nums.len() - 1 {
            if nums[index + 1] - nums[index] <= threshold {
                count += 1;
                index += 1;
            }
            index += 1;
        }
        count >= p
    }
    nums.sort();
    let (mut left, mut right) = (0, nums[nums.len() - 1] - nums[0]);
    while left < right {
        let mid = left + (right - left) / 2;
        if is_valid(&nums, p, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}
