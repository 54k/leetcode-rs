// https://leetcode.com/problems/remove-interval/description/
pub fn remove_interval(intervals: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    for int in intervals {
        let s = int[0];
        let e = int[1];
        if e < to_be_removed[0] || s > to_be_removed[1] {
            ans.push(vec![s, e]);
        } else {
            if s < to_be_removed[0] {
                ans.push(vec![s, to_be_removed[0]]);
            }

            if e > to_be_removed[1] {
                ans.push(vec![to_be_removed[1], e]);
            }
        }
    }
    ans
}

// https://leetcode.com/problems/maximum-score-of-a-good-subarray/description/
pub fn maximum_score_bin_search(nums: Vec<i32>, k: i32) -> i32 {
    fn binary_search(nums: &[i32], target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() as i32;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    fn solve(nums: &[i32], k: i32) -> i32 {
        let n = nums.len() as i32;
        let mut left = vec![0; k as usize];
        let mut curr_min = i32::MAX;

        for i in (0..k).rev() {
            curr_min = curr_min.min(nums[i as usize]);
            left[i as usize] = curr_min;
        }

        let mut right = vec![];
        curr_min = i32::MAX;
        for i in k..n {
            curr_min = curr_min.min(nums[i as usize]);
            right.push(curr_min);
        }

        let mut ans = 0;
        for j in 0..right.len() {
            curr_min = right[j];
            let i = binary_search(&left, curr_min);
            let size = (k + j as i32) - i + 1;
            ans = ans.max(curr_min * size);
        }
        ans
    }

    let ans = solve(&nums, k);
    let mut nums = nums;
    nums.reverse();

    ans.max(solve(&nums, nums.len() as i32 - k - 1))
}

pub fn maximum_score_monotonic_stack(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut left = vec![-1; n];
    let mut stack = vec![];

    for i in (0..n).rev() {
        while !stack.is_empty() && nums[stack[stack.len() - 1] as usize] > nums[i] {
            left[stack.pop().unwrap() as usize] = i as i32;
        }
        stack.push(i as i32);
    }

    let mut right = vec![n as i32; n];
    stack.clear();

    for i in 0..n {
        while !stack.is_empty() && nums[stack[stack.len() - 1] as usize] > nums[i] {
            right[stack.pop().unwrap() as usize] = i as i32;
        }
        stack.push(i as i32);
    }

    let mut ans = 0;
    for i in 0..n {
        if left[i] < k && right[i] > k {
            ans = ans.max(nums[i] * (right[i] - left[i] - 1));
        }
    }

    ans
}

pub fn maximum_score_greedy(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut left = k as usize;
    let mut right = k as usize;
    let mut ans = nums[k as usize];
    let mut curr_min = nums[k as usize];

    while left > 0 || right < n - 1 {
        let next_left = if left > 0 { nums[left - 1] } else { 0 };

        let next_right = if right < n - 1 { nums[right + 1] } else { 0 };

        if next_left < next_right {
            right += 1;
            curr_min = curr_min.min(nums[right]);
        } else {
            left -= 1;
            curr_min = curr_min.min(nums[left]);
        }

        ans = ans.max(curr_min * (right - left + 1) as i32);
    }

    ans
}

// https://leetcode.com/problems/constrained-subsequence-sum/description/
pub fn constrained_subset_sum_heap(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();

    heap.push((nums[0], 0));
    let mut ans = nums[0];

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

pub fn constrained_subset_sum_tree_map(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BTreeMap;

    let mut window = BTreeMap::new();
    window.insert(0, 0);

    let mut dp = vec![0; nums.len()];

    for i in 0..nums.len() {
        dp[i] = nums[i] + window.iter().last().unwrap().0;
        *window.entry(dp[i]).or_insert(0) += 1;

        if i >= k as usize {
            *window.entry(dp[i - k as usize]).or_insert(0) -= 1;
            if *window.get(&dp[i - k as usize]).unwrap() == 0 {
                window.remove(&dp[i - k as usize]);
            }
        }
    }

    dp.into_iter().max().unwrap()
}

pub fn constrained_subset_sum_deque(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::VecDeque;

    let mut deque = VecDeque::new();
    let mut dp = vec![0; nums.len()];

    for i in 0..nums.len() {
        if !deque.is_empty() && i - deque[0] > k as usize {
            deque.pop_front();
        }

        dp[i] = nums[i] + if !deque.is_empty() { dp[deque[0]] } else { 0 };

        while !deque.is_empty() && dp[deque[deque.len() - 1]] < dp[i] {
            deque.pop_back();
        }

        if dp[i] > 0 {
            deque.push_back(i);
        }
    }

    dp.into_iter().max().unwrap()
}
