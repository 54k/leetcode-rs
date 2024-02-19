// https://leetcode.com/problems/power-of-two/description/
pub fn is_power_of_two(n: i32) -> bool {
    let n = n as i64;
    if n == 0 {
        return false;
    }
    (n & (n - 1)) == 0
}

