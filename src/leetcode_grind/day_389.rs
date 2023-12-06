// https://leetcode.com/problems/calculate-money-in-leetcode-bank/description
pub fn total_money_1(n: i32) -> i32 {
    let k = n / 7;
    let f = 28;
    let l = 28 + (k - 1) * 7;
    let arith_sum = k * (f + l) / 2;

    let moday = k + 1;
    let mut final_week = 0;
    for day in 0..n % 7 {
        final_week += moday + day;
    }

    arith_sum + final_week
}

pub fn total_money_2(n: i32) -> i32 {
    let mut n = n;
    let mut ans = 0;
    let mut monday = 1;
    while n > 0 {
        for day in 0..n.min(7) {
            ans += monday + day;
        }
        n -= 7;
        monday += 1;
    }

    ans
}
