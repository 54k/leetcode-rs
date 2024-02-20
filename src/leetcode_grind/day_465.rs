// https://leetcode.com/problems/missing-number/description/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut exp = nums.len() as i32 * (nums.len() as i32 + 1) / 2;
    let mut actual = 0;
    for n in nums {
        actual += n;
    }
    exp - actual
}
