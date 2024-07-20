// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/description/?envType=daily-question&envId=2024-07-20
pub fn restore_matrix_i(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = row_sum.len();
    let m = col_sum.len();

    let mut curr_row_sum = vec![0; n];
    let mut curr_col_sum = vec![0; m];

    let mut orig_matrix = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            orig_matrix[i][j] = (row_sum[i] - curr_row_sum[i]).min(col_sum[j] - curr_col_sum[j]);
            curr_row_sum[i] += orig_matrix[i][j];
            curr_col_sum[j] += orig_matrix[i][j];
        }
    }
    orig_matrix
}

pub fn restore_matrix_ii(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = row_sum.len();
    let m = col_sum.len();

    let mut orig_matrix = vec![vec![0; m]; n];
    let mut i = 0;
    let mut j = 0;

    while i < n && j < m {
        orig_matrix[i][j] = row_sum[i].min(col_sum[j]);

        row_sum[i] -= orig_matrix[i][j];
        col_sum[j] -= orig_matrix[i][j];

        if row_sum[i] == 0 {
            i += 1;
        } else {
            j += 1;
        }
    }

    orig_matrix
}
