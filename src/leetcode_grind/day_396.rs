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

// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/description/
pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut diff_grid = vec![vec![0; n]; m];

    let mut onesr = vec![0; m];
    let mut onesc = vec![0; n];

    for r in 0..m {
        for c in 0..n {
            onesr[r] += grid[r][c];
            onesc[c] += grid[r][c];
        }
    }

    for r in 0..m {
        for c in 0..n {
            diff_grid[r][c] = 2 * (onesr[r] + onesc[c]) - (m as i32 + n as i32);
        }
    }

    diff_grid
}
