// https://leetcode.com/problems/minimum-array-end/description/?envType=daily-question&envId=2024-11-09
pub fn min_end(mut n: i32, x: i32) -> i64 {
    let mut n = n as i64;
    let x = x as i64;
    let mut result = x as i64;
    let mut mask = 1;
    n -= 1;
    while n > 0 {
        if (mask & x) == 0 {
            result |= (n & 1) * mask;
            n >>= 1;
        }
        mask <<= 1;
    }
    result
}
