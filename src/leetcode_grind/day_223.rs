// https://leetcode.com/problems/longest-arithmetic-subsequence/
pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut max_len = 0;
    let mut dp = vec![HashMap::new(); nums.len()];

    for right in 0..nums.len() {
        dp[right] = HashMap::new();
        for left in 0..right {
            let diff = nums[left] - nums[right];
            let l = *dp[left].get(&diff).unwrap_or(&1) + 1;
            dp[right].insert(diff, l);
            max_len = max_len.max(dp[right][&diff]);
        }
    }
    max_len
}

// https://leetcode.com/problems/longest-arithmetic-subsequence/
pub fn destroy_targets(mut nums: Vec<i32>, space: i32) -> i32 {
    use std::collections::HashMap;
    let mut max = i32::MIN;
    let mut hm = HashMap::new();
    for i in 0..nums.len() {
        let k = nums[i] % space;
        *hm.entry(k).or_insert(0) += 1;
        if hm[&k] > max {
            max = hm[&k];
        }
    }
    let mut ans = i32::MAX;
    for i in 0..nums.len() {
        let k = nums[i] % space;
        if hm[&k] == max && ans > nums[i] {
            ans = nums[i];
        }
    }
    ans
}
