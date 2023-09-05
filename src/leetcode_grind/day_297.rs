// https://leetcode.com/problems/max-chunks-to-make-sorted/description/
pub fn max_chunks_to_sorted_i(arr: Vec<i32>) -> i32 {
    if arr.len() == 0 {
        return 0;
    }
    let mut count = 0;
    let mut max = vec![0; arr.len()];
    max[0] = arr[0];
    for i in 1..arr.len() {
        max[i] = max[i - 1].max(arr[i]);
    }

    for i in 0..arr.len() {
        if max[i] == i as i32 {
            count += 1;
        }
    }
    count
}

pub fn max_chunks_to_sorted_ii(arr: Vec<i32>) -> i32 {
    let mut cur = 0;
    let mut chunks = 0;
    for i in 0..arr.len() {
        cur += arr[i];
        if cur == (i as i32 * (i as i32 + 1)) / 2 {
            chunks += 1;
        }
    }
    chunks
}

pub fn max_chunks_to_sorted_iii(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut stack = vec![];

    for i in 0..n {
        let mut maxi = arr[i];
        while stack.len() > 0 && stack[stack.len() - 1] > arr[i] {
            maxi = maxi.max(stack.pop().unwrap());
        }
        stack.push(maxi);
    }
    stack.len() as i32
}

// https://leetcode.com/problems/sliding-window-maximum/description/
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut ans = vec![];

    for i in 0..nums.len() {
        while queue.len() > 0 && nums[queue[queue.len() - 1]] < nums[i] {
            queue.pop_back();
        }
        queue.push_back(i);

        while queue.len() > 0 && (queue[0] as i32) < i as i32 - k as i32 + 1 {
            queue.pop_front();
        }

        if i + 1 >= k as usize {
            ans.push(nums[queue[0]]);
        }
    }

    ans
}
