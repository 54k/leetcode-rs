// https://leetcode.com/problems/constrained-subsequence-sum/description
pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut dp = vec![0; nums.len()];

    for i in 0..nums.len() {
        if !queue.is_empty() && i - queue[0] > k as usize {
            queue.pop_front();
        }

        dp[i] = if !queue.is_empty() { dp[queue[0]] } else { 0 } + nums[i];

        while !queue.is_empty() && dp[queue[queue.len() - 1]] < dp[i] {
            queue.pop_back();
        }

        if dp[i] > 0 {
            queue.push_back(i);
        }
    }

    let mut ans = i32::MIN;
    for num in dp {
        ans = ans.max(num);
    }
    ans
}

pub fn constrained_subset_sum_heap(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push((nums[0], 0));

    let mut ans = 0;

    for i in 1..nums.len() {
        while i - heap.peek().unwrap().1 > k as usize {
            heap.pop();
        }

        let curr = 0.max(heap.peek().unwrap().0) + nums[i];
        heap.push((curr, i));
        ans = ans.max(curr);
    }

    ans
}
