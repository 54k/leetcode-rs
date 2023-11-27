// https://leetcode.com/problems/knight-dialer/description
pub fn knight_dialer_1(n: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let moves = vec![
        vec![4, 6],
        vec![6, 8],
        vec![7, 9],
        vec![4, 8],
        vec![0, 3, 9],
        vec![],
        vec![0, 1, 7],
        vec![2, 6],
        vec![1, 3],
        vec![2, 4],
    ];

    let mut dp = vec![1; 10];
    for _ in 1..n as usize {
        let mut next = vec![0; 10];
        for j in 0..=9 {
            for &from in &moves[j] {
                next[j] = (next[j] + dp[from]) % MOD;
            }
        }
        dp = next;
    }

    let mut ans = 0;
    for i in 0..=9 {
        ans = (ans + dp[i]) % MOD;
    }
    ans
}
