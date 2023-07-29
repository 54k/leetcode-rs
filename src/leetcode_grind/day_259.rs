// https://leetcode.com/problems/soup-servings/description/
pub fn soup_servings(n: i32) -> f64 {
    if n > 5000 {
        return 1.0;
    }
    fn calc_dp(i: i32, j: i32, dp: &mut Vec<Vec<f64>>) -> f64 {
        (dp[0.max(i - 4) as usize][j as usize]
            + dp[0.max(i - 3) as usize][j as usize - 1]
            + dp[0.max(i - 2) as usize][0.max(j - 2) as usize]
            + dp[i as usize - 1][0.max(j - 3) as usize])
            / 4.0
    }
    let m = (n as f64 / 25.0).ceil() as usize;
    let mut dp = vec![vec![0.0; m + 1]; m + 1];
    dp[0][0] = 0.5;

    for k in 1..=m {
        dp[0][k] = 1.0;
        dp[k][0] = 0.0;

        for j in 1..=k {
            dp[j][k] = calc_dp(j as i32, k as i32, &mut dp);
            dp[k][j] = calc_dp(k as i32, j as i32, &mut dp);
        }

        if dp[k][k] > 1.0 - 1e-5 {
            return 1.0;
        }
    }

    dp[m][m]
}
