// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/
pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let mut l = 0;
    let mx = *nums.iter().max().unwrap();
    let mut curr = 0;
    let mut ans = 0;

    for r in 0..nums.len() {
        if nums[r] == mx {
            curr += 1;
        }

        while curr == k {
            if nums[l] == mx {
                curr -= 1;
            }
            l += 1;
        }

        ans += l as i64;
    }

    ans
}
