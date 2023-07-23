// https://leetcode.com/problems/path-with-minimum-effort/description/
pub fn minimum_effort_path_brute_tle(mut heights: Vec<Vec<i32>>) -> i32 {
    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut max_so_far = i32::MAX;

    fn is_valid_cell(x: i32, y: i32, row: usize, col: usize) -> bool {
        x >= 0 && x <= row as i32 - 1 && y >= 0 && y <= col as i32 - 1
    }

    fn backtrack(
        x: i32,
        y: i32,
        heights: &mut Vec<Vec<i32>>,
        row: usize,
        col: usize,
        max_diff: i32,
        max_so_far: &mut i32,
    ) -> i32 {
        if x as usize == row - 1 && y as usize == col - 1 {
            *max_so_far = (*max_so_far).min(max_diff);
            return max_diff;
        }

        let curr_height = heights[x as usize][y as usize];
        heights[x as usize][y as usize] = 0;
        let mut min_effort = i32::MAX;

        for i in 0..4 {
            let adj_x = x + DIRS[i].0;
            let adj_y = y + DIRS[i].1;

            if is_valid_cell(adj_x, adj_y, row, col) && heights[adj_x as usize][adj_y as usize] != 0
            {
                let curr_diff = (heights[adj_x as usize][adj_y as usize] - curr_height).abs();
                let max_curr_diff = max_diff.max(curr_diff);

                if max_curr_diff < *max_so_far {
                    let res = backtrack(adj_x, adj_y, heights, row, col, max_curr_diff, max_so_far);
                    min_effort = min_effort.min(res);
                }
            }
        }

        heights[x as usize][y as usize] = curr_height;
        min_effort
    }
    let (row, col) = (heights.len(), heights[0].len());
    backtrack(0, 0, &mut heights, row, col, 0, &mut max_so_far)
}

pub fn minimum_effort_path_dijkstras(heights: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    fn is_valid(x: i32, y: i32, row: usize, col: usize) -> bool {
        0 <= x && 0 <= y && row as i32 > x && col as i32 > y
    }

    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let (row, col) = (heights.len(), heights[0].len());

    let mut diff_matrix = vec![vec![i32::MAX; col]; row];
    diff_matrix[0][0] = 0;

    let mut visited = vec![vec![false; col]; row];

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((diff_matrix[0][0], 0, 0)));

    while let Some(Reverse(curr)) = heap.pop() {
        visited[curr.1 as usize][curr.2 as usize] = true;

        if curr.1 as usize == row - 1 && curr.2 as usize == col - 1 {
            return curr.0;
        }

        for dir in &DIRS {
            let adj_x = curr.1 + dir.0;
            let adj_y = curr.2 + dir.1;

            if is_valid(adj_x, adj_y, row, col) && !visited[adj_x as usize][adj_y as usize] {
                let curr_diff = (heights[adj_x as usize][adj_y as usize]
                    - heights[curr.1 as usize][curr.2 as usize])
                    .abs();
                let max_diff = curr_diff.max(diff_matrix[curr.1 as usize][curr.2 as usize]);
                if diff_matrix[adj_x as usize][adj_y as usize] > max_diff {
                    diff_matrix[adj_x as usize][adj_y as usize] = max_diff;
                    heap.push(Reverse((max_diff, adj_x, adj_y)));
                }
            }
        }
    }

    diff_matrix[row - 1][col - 1]
}
