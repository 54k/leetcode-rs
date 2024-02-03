    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![0; n + 1];

        for start in (0..n).rev() {
            let mut mx = 0;
            let end = (start + k as usize).min(n);

            for i in start..end {
                mx = mx.max(arr[i]);
                dp[start] = dp[start].max(dp[i+1] + mx * (i - start + 1) as i32);
            }
        }
        dp[0]
    }