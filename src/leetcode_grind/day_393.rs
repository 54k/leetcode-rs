// https://leetcode.com/problems/transpose-matrix/description
pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut ans = vec![vec![0; m]; n];
    for r in 0..m {
        for c in 0..n {
            ans[c][r] = matrix[r][c];
        }
    }
    ans
}
