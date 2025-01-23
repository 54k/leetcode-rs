// https://leetcode.com/problems/count-servers-that-communicate/description/?envType=daily-question&envId=2025-01-23
pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() == 0 {
        return 0;
    }

    let mut row_count = vec![0; grid[0].len()];
    let mut col_count = vec![0; grid.len()];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 1 {
                row_count[col] += 1;
                col_count[row] += 1;
            }
        }
    }

    let mut communicable_servers_count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 1 {
                if row_count[col] > 1 || col_count[row] > 1 {
                    communicable_servers_count += 1;
                }
            }
        }
    }
    communicable_servers_count
}

// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-i/description/
pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut k = 0;
    for i in 0..m {
        let mut c = 0;
        let mut l = 0 as i32;
        let mut r = n as i32 - 1;

        while l < r {
            if grid[i][l as usize] != grid[i][r as usize] {
                c += 1;
            }
            l += 1;
            r -= 1;
        }
        k += c;
    }

    let mut c = 0;
    for i in 0..n {
        let mut l = 0i32;
        let mut r = m as i32 - 1;
        while l < r {
            if grid[l as usize][i] != grid[r as usize][i] {
                c += 1;
            }
            l += 1;
            r -= 1;
        }
    }

    k.min(c)
}
