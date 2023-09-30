// https://leetcode.com/problems/132-pattern/description
pub fn find132pattern_search_interval(nums: Vec<i32>) -> bool {
    let mut intervals = vec![];
    let mut s = 0;

    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            if s < i - 1 {
                intervals.push((s, i - 1));
            }
            s = i;
        }

        for a in &intervals {
            if nums[i] < nums[a.1] && nums[a.0] < nums[i] {
                return true;
            }
        }
    }
    false
}

pub fn find132pattern_stacks(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }
    let mut stack = vec![];

    let mut min = vec![];
    min.push(nums[0]);
    for i in 1..nums.len() {
        min.push(min[i - 1].min(nums[i]));
    }

    for j in (0..nums.len()).rev() {
        if nums[j] > min[j] {
            while !stack.is_empty() && stack[stack.len() - 1] <= min[j] {
                stack.pop();
            }
            if !stack.is_empty() && stack[stack.len() - 1] < nums[j] {
                return true;
            }
            stack.push(nums[j]);
        }
    }
    false
}
