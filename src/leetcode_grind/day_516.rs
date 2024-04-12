// https://leetcode.com/problems/trapping-rain-water/description/
pub fn trap(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                ans += left_max - height[left];
            }
            left += 1
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                ans += right_max - height[right];
            }
            right -= 1;
        }
    }

    ans
}
