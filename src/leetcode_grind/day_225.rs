// https://leetcode.com/problems/count-all-possible-routes/description/
pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
    let n = locations.len();
    let mut dp = vec![vec![0; fuel as usize + 1]; n];
    dp[finish as usize] = vec![1; fuel as usize + 1];

    for j in 0..=fuel as usize {
        for i in 0..n {
            for k in 0..n {
                if i == k {
                    continue;
                }
                let abs = (locations[i] - locations[k]).abs() as usize;
                if abs <= j {
                    dp[i][j] = (dp[i][j] + dp[k][j - abs]) % 1000000007
                }
            }
        }
    }

    dp[start as usize][fuel as usize]
}
