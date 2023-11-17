// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/
pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut ans = 0;
    let (mut i, mut j) = (0, nums.len() - 1);
    while i < j {
        ans = ans.max(nums[i] + nums[j]);
        i += 1;
        j -= 1;
    }
    ans
}
