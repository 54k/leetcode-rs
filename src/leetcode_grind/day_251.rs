// https://leetcode.com/problems/largest-divisible-subset/description/
pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut nums = nums;
    nums.sort();
    let mut eds = vec![vec![]; n];
    for i in 0..nums.len() {
        let mut max_subset = vec![];

        for j in 0..i {
            if nums[i] % nums[j] == 0 && max_subset.len() < eds[j].len() {
                max_subset = eds[j].clone();
            }
        }

        eds[i].extend(max_subset);
        eds[i].push(nums[i]);
    }

    let mut ans = vec![];
    for i in 0..n {
        if ans.len() < eds[i].len() {
            ans = eds[i].clone();
        }
    }
    ans
}

// https://leetcode.com/problems/distinct-subsequences/description/
pub fn num_distinct(s: String, t: String) -> i32 {
    let m = s.len();
    let n = t.len();
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][n] = 1;
    }

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = dp[i + 1][j];

            if s[i] == t[j] {
                dp[i][j] += dp[i + 1][j + 1];
            }
        }
    }

    dp[0][0]
}
