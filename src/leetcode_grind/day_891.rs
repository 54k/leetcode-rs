// https://leetcode.com/problems/count-subarrays-with-score-less-than-k/description/?envType=daily-question&envId=2025-04-28
pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let n = nums.len();        
    let mut res = 0;
    let mut total = 0;
    let mut i = 0;
    for j in 0..n {
        total += nums[j] as i64;
        while i <= j && total * (j - i + 1) as i64 >= k {
            total -= nums[i] as i64;
            i += 1;
        }
        res += j - i + 1;
    }
    res as i64
}