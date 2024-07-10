// https://leetcode.com/problems/crawler-log-folder/description/?envType=daily-question&envId=2024-07-10
pub fn min_operations_1(logs: Vec<String>) -> i32 {
    let mut folder_depth = 0;
    for current_operation in logs {
        if current_operation == "../" {
            folder_depth = 0.max(folder_depth - 1);
        } else if (current_operation != "./") {
            folder_depth += 1;
        }
    }
    folder_depth
}

// https://leetcode.com/problems/max-increase-to-keep-city-skyline/description/
pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut row_maxes = vec![0; n];
    let mut col_maxes = vec![0; n];

    for r in 0..n {
        for c in 0..n {
            row_maxes[r] = row_maxes[r].max(grid[r][c]);
            col_maxes[c] = col_maxes[c].max(grid[r][c]);
        }
    }

    let mut ans = 0;
    for r in 0..n {
        for c in 0..n {
            ans += row_maxes[r].min(col_maxes[c]) - grid[r][c];
        }
    }
    ans
}
