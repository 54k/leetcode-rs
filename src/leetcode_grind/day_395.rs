// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/description
pub fn max_product(nums: Vec<i32>) -> i32 {
    let (mut mx1, mut mx2) = (0, 0);
    for i in 0..nums.len() {
        if nums[i] > mx1 {
            mx2 = mx1;
            mx1 = nums[i];
        } else if nums[i] > mx2 {
            mx2 = nums[i];
        }
    }

    (mx1 - 1) * (mx2 - 1)
}
