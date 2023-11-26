// https://leetcode.com/problems/largest-submatrix-with-rearrangements/description
pub fn largest_submatrix_1(matrix: Vec<Vec<i32>>) -> i32 {
    let mut matrix = matrix;
    let mut ans = 0;
    let m = matrix.len();
    let n = matrix[0].len();
    for r in 0..m {
        for c in 0..n {
            if matrix[r][c] != 0 && r > 0 {
                matrix[r][c] += matrix[r - 1][c];
            }
        }
        let mut curr_row = matrix[r].clone();
        curr_row.sort();
        for i in 0..n {
            ans = ans.max(curr_row[i] * (n - i) as i32);
        }
    }
    ans
}

pub fn largest_submatrix_2(matrix: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let m = matrix.len();
    let n = matrix[0].len();

    let mut prev_row = vec![0; n];

    for r in 0..m {
        let mut curr_row = matrix[r].clone();

        for c in 0..n {
            if curr_row[c] != 0 {
                curr_row[c] += prev_row[c];
            }
        }

        let mut sorted_row = curr_row.clone();
        sorted_row.sort();
        for i in 0..n {
            ans = ans.max(sorted_row[i] * (n - i) as i32);
        }

        prev_row = curr_row;
    }

    ans
}

pub fn largest_submatrix_3(matrix: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let m = matrix.len();
    let n = matrix[0].len();

    let mut prev_heights: Vec<(i32, usize)> = vec![];

    for row in 0..m {
        let mut heights = vec![];
        let mut seen = vec![false; n];

        for (height, col) in &prev_heights {
            if matrix[row][*col] == 1 {
                heights.push((*height + 1, *col));
                seen[*col] = true;
            }
        }

        for col in 0..n {
            if !seen[col] && matrix[row][col] == 1 {
                heights.push((1, col));
            }
        }

        for i in 0..heights.len() {
            ans = ans.max(heights[i].0 * (i + 1) as i32);
        }

        prev_heights = heights;
    }

    ans
}
