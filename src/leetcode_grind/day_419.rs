// https://leetcode.com/problems/longest-increasing-subsequence/
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![];

    for i in 0..nums.len() {
        if dp.is_empty() || *dp.last().unwrap() < nums[i] {
            dp.push(nums[i]);
        } else {
            let (mut lo, mut hi) = (0, dp.len());
            while lo < hi {
                let mid = (lo + hi) / 2;
                if dp[mid] < nums[i] {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            dp[lo] = nums[i];
        }
    }

    dp.len() as i32
}
