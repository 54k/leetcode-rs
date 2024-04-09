// https://leetcode.com/problems/wiggle-subsequence/description
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    let (mut up, mut down) = (1, 1);
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            up = up.max(down + 1);
        } else if nums[i] < nums[i - 1] {
            down = down.max(up + 1)
        }
    }

    up.max(down)
}
