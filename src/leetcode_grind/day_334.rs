// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/description/
pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let mut left = vec![1; nums.len()];
    for i in 0..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                left[i] = left[i].max(1 + left[j]);
            }
        }
    }

    let mut right = vec![1; nums.len()];
    for i in (0..n).rev() {
        for j in (i + 1..n).rev() {
            if nums[i] > nums[j] {
                right[i] = right[i].max(1 + right[j]);
            }
        }
    }

    let mut max_len = 0;
    for i in 0..n {
        if left[i] >= 2 && right[i] >= 2 {
            max_len = max_len.max(left[i] + right[i] - 1);
        }
    }

    n as i32 - max_len
}
