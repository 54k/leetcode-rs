// https://leetcode.com/problems/count-ways-to-build-good-strings/description/
pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    const MOD: i64 = 1000_000_007;
    let mut dp = vec![-1; high as usize + 1];
    dp[0] = 1;
    fn rec((lo, hi): (i32, i32), zero: i32, one: i32, len: usize, dp: &mut Vec<i64>) -> i64 {
        if len > hi as usize {
            return 0;
        }
        if dp[len] == -1 {
            let mut sum = rec((lo, hi), zero, one, len + zero as usize, dp)
                + rec((lo, hi), zero, one, len + one as usize, dp);
            if len >= lo as usize && len <= hi as usize {
                sum += 1
            }
            dp[len] = sum % MOD;
        }
        dp[len]
    }
    (rec((low, high), zero, one, 0, &mut dp) % MOD) as i32
}
