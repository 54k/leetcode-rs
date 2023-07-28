// https://leetcode.com/problems/predict-the-winner/description/
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    fn predict_the_winner_top_down(nums: Vec<i32>) -> bool {
        fn max_diff(nums: &Vec<i32>, left: usize, right: usize, n: usize) -> i32 {
            if left == right {
                return nums[left];
            }

            let score_by_left = nums[left] - max_diff(nums, left + 1, right, n);
            let score_by_right = nums[right] - max_diff(nums, left, right - 1, n);

            score_by_left.max(score_by_right)
        }

        let n = nums.len();
        max_diff(&nums, 0, n - 1, n) >= 0
    }

    fn predict_the_winner_bottom_up(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = nums[i];
        }

        for diff in 1..n {
            for left in 0..n - diff {
                let right = left + diff;
                dp[left][right] =
                    (nums[left] - dp[left + 1][right]).max(nums[right] - dp[left][right - 1]);
            }
        }

        dp[0][n - 1] >= 0
    }

    predict_the_winner_bottom_up(nums)
}
