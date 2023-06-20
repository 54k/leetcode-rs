// https://leetcode.com/problems/find-the-highest-altitude/description/
pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut cur = 0;
    for i in 0..gain.len() {
        cur += gain[i];
        ans = ans.max(cur);
    }
    ans
}
