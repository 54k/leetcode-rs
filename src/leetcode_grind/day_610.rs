// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/?envType=daily-question&envId=2024-07-20
pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut r_min_max = i32::MIN;
    for i in 0..n {
        let mut r_min = i32::MAX;
        for j in 0..m {
            r_min = r_min.min(matrix[i][j]);
        }
        r_min_max = r_min_max.max(r_min);
    }

    let mut c_max_min = i32::MAX;
    for i in 0..m {
        let mut c_max = i32::MIN;
        for j in 0..n {
            c_max = c_max.max(matrix[j][i]);
        }
        c_max_min = c_max_min.min(c_max);
    }

    if r_min_max == c_max_min {
        vec![r_min_max]
    } else {
        vec![]
    }
}
