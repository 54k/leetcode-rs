// https://leetcode.com/problems/find-missing-and-repeated-values/description/
pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut ans = vec![0, 0];
    let mut set = HashSet::new();
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..(m * n) {
        if !set.insert(grid[i / n][i % n]) {
            ans[0] = grid[i / n][i % n];
        }
    }
    for i in 0..(m * n) {
        if !set.contains(&(i as i32 + 1)) {
            ans[1] = i as i32 + 1;
            break;
        }
    }
    ans
}