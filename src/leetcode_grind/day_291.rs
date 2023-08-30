// https://leetcode.com/problems/maximal-range-that-each-element-is-maximum-in-it/
pub fn maximum_length_of_ranges(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![1; nums.len()];
    let n = nums.len();
    let mut stack = vec![];

    stack.push(-1);
    for i in 0..n {
        while stack[stack.len() - 1] != -1 && nums[stack[stack.len() - 1] as usize] < nums[i] {
            stack.pop();
        }
        ans[i] = i as i32 - stack[stack.len() - 1];
        stack.push(i as i32);
    }

    stack.clear();
    stack.push(n as i32);

    for i in (0..n).rev() {
        while (stack[stack.len() - 1] != n as i32)
            && nums[stack[stack.len() - 1] as usize] < nums[i]
        {
            stack.pop();
        }
        ans[i] += stack[stack.len() - 1] - i as i32 - 1;
        stack.push(i as i32);
    }

    ans
}