// https://leetcode.com/problems/find-the-count-of-monotonic-pairs-i/description/
pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let mut cnt = 0i64;
    let mut dp = vec![vec![0; 1001]; n];

    for i in 0..=nums[0] {
        dp[0][i as usize] = 1;
    }

    for i in 1..n {
        for prev in 0..=nums[i - 1] {
            for k in prev..=nums[i] {
                let val = nums[i] - k;
                if val <= nums[i - 1] - prev {
                    dp[i][k as usize] = (dp[i][k as usize] + dp[i - 1][prev as usize]) % MOD;
                }
            }
        }
    }

    for i in 0..=nums[n - 1] {
        cnt = (cnt + dp[n - 1][i as usize]) % MOD;
    }

    cnt as i32
}

// https://leetcode.com/problems/find-the-count-of-monotonic-pairs-ii/description/
pub fn count_of_pairs_ii(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = nums.len();

    let mut dp = vec![vec![0; 1001]; n];
    for j in 0..=nums[0] {
        dp[0][j as usize] = 1;
    }

    for i in 1..n {
        let mut ways = 0;
        let mut k = 0;
        for j in 0..=nums[i] {
            if k <= j.min(j - (nums[i] - nums[i - 1])) {
                ways = (ways + dp[i - 1][k as usize]) % MOD;
                k += 1;
            }
            dp[i][j as usize] = ways;
        }
    }

    let mut res = 0;
    for i in 0..=1000 {
        res = (res + dp[n - 1][i]) % MOD;
    }
    res
}
