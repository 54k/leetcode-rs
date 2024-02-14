// https://leetcode.com/problems/rearrange-array-elements-by-sign/description/
pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut p = 0;
    let mut n = 0;
    while res.len() != nums.len() {
        while nums[p] < 0 {
            p += 1;
        }
        while nums[n] > 0 {
            n += 1;
        }
        res.push(nums[p]);
        res.push(nums[n]);
        p += 1;
        n += 1;
    }
    res
}
