// https://leetcode.com/problems/trapping-rain-water/description/
pub fn trap_stack(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stack = vec![];
    let mut current = 0;

    while current < height.len() {
        while !stack.is_empty() && height[stack[stack.len() - 1]] <= height[current] {
            let mid = stack.pop().unwrap();
            if stack.is_empty() {
                break;
            }
            let distance = current - stack[stack.len() - 1] - 1;
            let bounded_height = height[current].min(height[stack[stack.len() - 1]]) - height[mid];
            ans += distance as i32 * bounded_height;
        }

        stack.push(current);
        current += 1;
    }

    ans
}

pub fn trap_2pointers(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut ans = 0;
    let mut left_max = 0;
    let mut right_max = 0;
    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                ans += left_max - height[left];
            }
            left += 1;
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
