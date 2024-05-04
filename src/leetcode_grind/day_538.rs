// https://leetcode.com/problems/maximum-number-of-operations-with-the-same-score-i/description/
pub fn max_operations(mut nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut prev = -1;
    for i in (0..=nums.len() - 2).step_by(2) {
        println!("{i}");
        let s = nums[i] + nums[i + 1];
        if prev == -1 {
            prev = s;
        } else if prev != s {
            break;
        }
        ans += 1;
    }
    ans
}
