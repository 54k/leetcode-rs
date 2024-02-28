// https://leetcode.com/problems/arranging-coins/description/
pub fn arrange_coins(n: i32) -> i32 {
    let mut left = 0i64;
    let mut right = n as i64;

    let mut k = 0;
    let mut curr = 0;

    while left <= right {
        k = left + (right - left) / 2;
        curr = k * (k + 1) / 2;

        if curr == n as i64 {
            return k as i32;
        }

        if (n as i64) < curr {
            right = k - 1;
        } else {
            left = k + 1;
        }
    }
    return right as i32;
}
