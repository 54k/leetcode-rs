// https://leetcode.com/problems/integer-break/description
pub fn integer_break(n: i32) -> i32 {
    let mut n = n;
    if n <= 3 {
        return n - 1;
    }

    let mut ans = 1;
    while n > 4 {
        ans *= 3;
        n -= 3;
    }

    ans * n
}
