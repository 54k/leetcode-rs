// https://leetcode.com/problems/sliding-window-maximum/description/
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut ans = vec![];

    for right in 0..nums.len() {
        while !queue.is_empty() && nums[right] > nums[queue[queue.len() - 1]] {
            queue.pop_back();
        }
        queue.push_back(right);

        while !queue.is_empty() && (queue[0] as i32) < right as i32 - k as i32 + 1 {
            queue.pop_front();
        }

        if right + 1 >= k as usize {
            ans.push(nums[queue[0]]);
        }
    }
    ans
}
