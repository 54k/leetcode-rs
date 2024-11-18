// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/description/?envType=daily-question&envId=2024-11-17
pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();

    let mut prefix_sums = vec![0; n + 1];

    for i in 1..=n {
        prefix_sums[i] = prefix_sums[i - 1] as i64 + nums[i - 1] as i64;
    }

    use std::collections::VecDeque;

    let mut candidate_indices = VecDeque::new();
    let mut shortest_subarray_len = i32::MAX;

    for i in 0..=n {
        while !candidate_indices.is_empty()
            && prefix_sums[i] - prefix_sums[candidate_indices[0]] >= k as i64
        {
            shortest_subarray_len =
                shortest_subarray_len.min(i as i32 - candidate_indices.pop_front().unwrap() as i32);
        }

        while !candidate_indices.is_empty()
            && prefix_sums[i] <= prefix_sums[candidate_indices[candidate_indices.len() - 1]]
        {
            candidate_indices.pop_back();
        }

        candidate_indices.push_back(i);
    }

    if shortest_subarray_len == i32::MAX {
        -1
    } else {
        shortest_subarray_len
    }
}
