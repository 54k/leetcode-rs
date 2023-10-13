// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/
pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let ones = nums.iter().copied().sum::<i32>();
    let mut ans = i32::MAX;
    let mut curr = 0;
    let mut j = 0;
    for i in 0..nums.len() * 2 {
        curr += nums[i % nums.len()];
        if i >= ones as usize {
            curr -= nums[j % nums.len()];
            j += 1;
        }
        ans = ans.min(ones - curr);
    }
    ans
}
