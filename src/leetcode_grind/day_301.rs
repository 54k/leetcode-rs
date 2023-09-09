// https://leetcode.com/problems/combination-sum-iv/description
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = vec![0; target as usize + 1];
    dp[0] = 1;
    for sum in 0..=target as usize {
        for &n in &nums {
            if sum >= n as usize {
                dp[sum as usize] += dp[sum as usize - n as usize];
            }
        }
    }
    dp[target as usize]
}
