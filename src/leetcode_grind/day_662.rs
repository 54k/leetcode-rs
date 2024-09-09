// https://leetcode.com/problems/number-of-digit-one/description/?envType=problem-list-v2&envId=dynamic-programming
pub fn count_digit_one(n: i32) -> i32 {
    let mut countr = 0;
    let mut i = 1;
    while i <= n {
        let divider = i * 10;
        countr += (n / divider) * i + (n % divider - i + 1).max(0).min(i);
        i *= 10;
    }
    countr
}
