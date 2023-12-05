// https://leetcode.com/problems/count-of-matches-in-tournament/description/
pub fn number_of_matches_1(n: i32) -> i32 {
    let mut n = n;
    let mut ans = 0;
    while n != 1 {
        if n % 2 == 0 {
            ans += n / 2;
            n /= 2;
        } else {
            ans += (n - 1) / 2;
            n = (n - 1) / 2 + 1;
        }
    }
    ans
}

pub fn number_of_matches_2(n: i32) -> i32 {
    n - 1
}
