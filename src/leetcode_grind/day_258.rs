// https://leetcode.com/problems/predict-the-winner/description/
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
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
