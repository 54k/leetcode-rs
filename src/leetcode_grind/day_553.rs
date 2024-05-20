// https://leetcode.com/problems/sum-of-all-subset-xor-totals/description
pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for mask in 1..(1 << n) {
        let mut cur = 0;
        for i in 0..n {
            if ((1 << i) & mask) > 0 {
                cur ^= nums[i];
            }
        }
        ans += cur;
    }
    ans
}
