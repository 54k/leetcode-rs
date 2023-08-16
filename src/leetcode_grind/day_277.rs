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

// https://leetcode.com/problems/determine-if-two-events-have-conflict/description/
pub fn have_conflict_1(event1: Vec<String>, event2: Vec<String>) -> bool {
    if event1[0] > event2[0] {
        return have_conflict_1(event2, event1);
    }
    event1[1] >= event2[0]
}

pub fn have_conflict_2(event1: Vec<String>, event2: Vec<String>) -> bool {
    let r = event1[1].clone().min(event2[1].clone());
    let l = event1[0].clone().max(event2[0].clone());
    l <= r
}
