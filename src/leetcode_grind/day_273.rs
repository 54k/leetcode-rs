// https://leetcode.com/problems/unique-paths-ii/description/
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    let mut dp = vec![0; n];
    dp[0] = 1;
    for row in 0..m {
        let mut next = vec![0; n];
        for col in 0..n {
            if obstacle_grid[row][col] == 0 {
                next[col] += dp[col];
                if col > 0 && obstacle_grid[row][col - 1] == 0 {
                    next[col] += next[col - 1];
                }
            }
        }
        dp = next;
    }
    dp[n - 1]
}

// https://leetcode.com/problems/describe-the-painting/description/
pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
    let mut mapping = vec![0; 100002];
    let mut ends = vec![false; 100002];
    for seg in segments {
        let (s, e, v) = (seg[0] as usize, seg[1] as usize, seg[2] as i64);
        mapping[s] += v;
        mapping[e] -= v;
        ends[s] = true;
        ends[e] = true;
    }

    let mut res = vec![];
    let mut prev = 0;
    let mut sum = 0;

    for i in 0..mapping.len() {
        if ends[i] && sum > 0 {
            res.push(vec![prev as i64, i as i64, sum]);
        }
        sum += mapping[i];
        if ends[i] {
            prev = i;
        }
    }
    res
}
