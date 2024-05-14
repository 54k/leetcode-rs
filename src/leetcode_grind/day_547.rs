// https://leetcode.com/problems/maximum-difference-score-in-a-grid/description/
pub fn max_score(mut grid: Vec<Vec<i32>>) -> i32 {
    let (mut res, m, n) = (i32::MIN, grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            let pre = if i > 0 { grid[i - 1][j] } else { i32::MAX }.min(if j > 0 {
                grid[i][j - 1]
            } else {
                i32::MAX
            });

            res = res.max(grid[i][j] - pre);
            grid[i][j] = grid[i][j].min(pre);
        }
    }
    res
}
