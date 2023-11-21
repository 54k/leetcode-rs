// https://leetcode.com/problems/island-perimeter/description/
pub fn island_perimeter_1(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            let cell = grid[i][j];
            if cell == 0 {
                continue;
            }

            let mut perimeter = 4;

            if i > 0 {
                perimeter -= grid[i - 1][j];
            }
            if i < m - 1 {
                perimeter -= grid[i + 1][j];
            }

            if j > 0 {
                perimeter -= grid[i][j - 1];
            }
            if j < n - 1 {
                perimeter -= grid[i][j + 1];
            }

            ans += perimeter;
        }
    }

    ans
}

pub fn island_perimeter_2(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            let cell = grid[i][j];
            if cell == 0 {
                continue;
            }

            let mut perimeter = 4;

            if i < m - 1 {
                perimeter -= grid[i + 1][j] * 2;
            }

            if j > 0 {
                perimeter -= grid[i][j - 1] * 2;
            }

            ans += perimeter;
        }
    }

    ans
}
