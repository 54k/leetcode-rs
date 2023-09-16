// https://leetcode.com/problems/path-with-minimum-effort/description
pub fn minimum_effort_path_i_tle(heights: Vec<Vec<i32>>) -> i32 {
    let mut heights = heights;
    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn is_valid_cell(x: i32, y: i32, row: i32, col: i32) -> bool {
        x >= 0 && x < row && y >= 0 && y < col
    }

    fn backtrack(
        x: i32,
        y: i32,
        heights: &mut Vec<Vec<i32>>,
        row: i32,
        col: i32,
        max_difference: i32,
        max_so_far: &mut i32,
    ) -> i32 {
        if x == row - 1 && y == col - 1 {
            *max_so_far = (*max_so_far).min(max_difference);
            return max_difference;
        }
        let cur_height = heights[x as usize][y as usize];
        heights[x as usize][y as usize] = 0;

        let mut min_effort = i32::MAX;

        for d in &DIRS {
            let nx = x + d.0;
            let ny = y + d.1;

            if is_valid_cell(nx, ny, row, col) && heights[nx as usize][ny as usize] != 0 {
                let cur_diff = i32::abs(cur_height - heights[nx as usize][ny as usize]);
                let max_cur_diff = max_difference.max(cur_diff);

                if max_cur_diff <= *max_so_far {
                    let result = backtrack(nx, ny, heights, row, col, max_cur_diff, max_so_far);
                    min_effort = min_effort.min(result);
                }
            }
        }

        heights[x as usize][y as usize] = cur_height;
        min_effort
    }

    let mut max_so_far = i32::MAX;
    let row = heights.len() as i32;
    let col = heights[0].len() as i32;
    backtrack(0, 0, &mut heights, row, col, 0, &mut max_so_far)
}

pub fn minimum_effort_path_ii_dijkstra(heights: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    fn is_valid_move(x: i32, y: i32, row: i32, col: i32) -> bool {
        x >= 0 && x < row && y >= 0 && y < col
    }

    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let row = heights.len() as i32;
    let col = heights[0].len() as i32;

    let mut diff_matrix = vec![vec![i32::MAX; col as usize]; row as usize];
    diff_matrix[0][0] = 0;
    let mut visited = vec![vec![false; col as usize]; row as usize];

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((diff_matrix[0][0], 0, 0)));

    while let Some(Reverse((w, x, y))) = heap.pop() {
        visited[x as usize][y as usize] = true;
        if x == row - 1 && y == col - 1 {
            return w;
        }

        for d in &DIRS {
            let nx = x + d.0;
            let ny = y + d.1;

            if is_valid_move(nx, ny, row, col) && !visited[nx as usize][ny as usize] {
                let curr_diff =
                    i32::abs(heights[x as usize][y as usize] - heights[nx as usize][ny as usize]);
                let max_diff = curr_diff.max(diff_matrix[x as usize][y as usize]);

                if diff_matrix[nx as usize][ny as usize] > max_diff {
                    diff_matrix[nx as usize][ny as usize] = max_diff;
                    heap.push(Reverse((max_diff, nx, ny)));
                }
            }
        }
    }

    diff_matrix[row as usize - 1][col as usize - 1]
}

#[test]
fn test_min_effort_path() {
    let res = minimum_effort_path_ii_dijkstra(vec![
        vec![1, 2, 1, 1, 1],
        vec![1, 2, 1, 2, 1],
        vec![1, 2, 1, 2, 1],
        vec![1, 2, 1, 2, 1],
        vec![1, 1, 1, 2, 1],
    ]);
    println!("{res}");

    let res = minimum_effort_path_ii_dijkstra(vec![vec![1, 10, 6, 7, 9, 10, 4, 9]]);
    println!("{res}");
}
