// https://leetcode.com/problems/special-positions-in-a-binary-matrix/description
pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (mat.len(), mat[0].len());
    let mut row_count = vec![0; m];
    let mut col_count = vec![0; n];

    for r in 0..m {
        for c in 0..n {
            if mat[r][c] == 1 {
                row_count[r] += 1;
                col_count[c] += 1;
            }
        }
    }

    let mut ans = 0;
    for r in 0..m {
        for c in 0..n {
            if mat[r][c] == 1 {
                if row_count[r] == 1 && col_count[c] == 1 {
                    ans += 1;
                }
            }
        }
    }

    ans
}
