// https://leetcode.com/problems/sum-of-square-numbers/description
pub fn judge_square_sum(c: i32) -> bool {
    let mut a = 0;
    while a * a <= c {
        let mut b = c as i64 - (a as i64 * a as i64);

        let mut lo = 0i64;
        let mut hi = b;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if mid * mid == b {
                return true;
            }
            if mid * mid < b {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        a += 1;
    }
    false
}
