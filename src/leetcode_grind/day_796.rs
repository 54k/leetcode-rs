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
