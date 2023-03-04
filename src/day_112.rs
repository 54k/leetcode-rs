// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/description/
// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/editorial/
pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut ans = 0;
    let mut min_pos = -1;
    let mut max_pos = -1;
    let mut left_bound = -1;
    for i in 0..nums.len() {
        if nums[i] < min_k || nums[i] > max_k {
            left_bound = i as i64;
        }
        if nums[i] == min_k {
            min_pos = i as i64;
        }
        if nums[i] == max_k {
            max_pos = i as i64;
        }

        ans += 0.max(max_pos.min(min_pos) - left_bound);
    }
    ans
}
