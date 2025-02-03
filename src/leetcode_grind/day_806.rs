// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/description/
pub fn check(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n <= 1 {
        return true;
    }

    let mut inversion_count = 0;
    for i in 1..n {
        if nums[i] < nums[i - 1] {
            inversion_count += 1;
        }
    }

    if (nums[0] < nums[n - 1]) {
        inversion_count += 1;
    }

    inversion_count <= 1
}
