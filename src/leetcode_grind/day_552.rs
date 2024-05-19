// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description
pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
    let mut sum = 0i64;
    let mut count = 0;

    let mut positive_min = (1 << 30);
    let mut negative_max = -(1 << 30);

    for &node_val in &nums {
        let mut operated_node_value = node_val ^ k;
        sum += node_val as i64;
        let mut net_change = operated_node_value - node_val;

        if net_change > 0 {
            positive_min = positive_min.min(net_change);
            sum += net_change as i64;
            count += 1;
        } else {
            negative_max = negative_max.max(net_change);
        }
    }

    if count % 2 == 0 {
        return sum;
    }

    return (sum - positive_min as i64).max(sum + negative_max as i64);
}

// https://leetcode.com/problems/edit-distance/description/
pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();

    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i as i32;
        for j in 0..=n {
            dp[0][j] = j as i32;
            if (i == 0 || j == 0) {
                continue;
            }

            if word1[i - 1] == word2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                let a = dp[i - 1][j];
                let b = dp[i][j - 1];
                let c = dp[i - 1][j - 1];
                dp[i][j] = a.min(b).min(c) + 1;
            }
        }
    }

    dp[m][n]
}
