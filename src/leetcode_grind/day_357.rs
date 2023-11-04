// https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/description/
pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut ans = 0;
    for num in left {
        ans = ans.max(num);
    }
    for num in right {
        ans = ans.max(n - num);
    }
    ans
}
