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

pub fn minimum_effort_path_iii_dsu(heights: Vec<Vec<i32>>) -> i32 {
    struct UF {
        repr: Vec<usize>,
        size: Vec<i32>,
    }

    impl UF {
        fn new(n: usize) -> Self {
            let mut repr = vec![];
            for i in 0..n {
                repr.push(i);
            }
            Self {
                repr,
                size: vec![1; n],
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.repr[x] != x {
                self.repr[x] = self.find(self.repr[x]);
            }
            self.repr[x]
        }

        fn union(&mut self, x: usize, y: usize) {
            let (mut x, mut y) = (self.find(x), self.find(y));

            if x == y {
                return;
            }

            if self.size[x] > self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }

            self.repr[x] = y;
            self.size[y] += self.size[x];
        }
    }

    let row = heights.len();
    let col = heights[0].len();
    let mut edges = vec![(0, 0, 0); row * col];

    fn fill_edges(
        edges: &mut Vec<(usize, usize, i32)>,
        heights: &Vec<Vec<i32>>,
        row: usize,
        col: usize,
    ) {
        for i in 0..row {
            for j in 0..col {
                if i > 0 {
                    edges.push((
                        i * col + j,
                        (i - 1) * col + j,
                        i32::abs(heights[i][j] - heights[i - 1][j]),
                    ));
                }

                if j > 0 {
                    edges.push((
                        i * col + j,
                        i * col + j - 1,
                        i32::abs(heights[i][j] - heights[i][j - 1]),
                    ));
                }
            }
        }
    }

    fill_edges(&mut edges, &heights, row, col);
    edges.sort_by_key(|e| e.2);
    let mut uf = UF::new(row * col);

    for (x, y, diff) in edges {
        uf.union(x, y);
        if uf.find(0) == uf.find(row * col - 1) {
            return diff;
        }
    }

    -1
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

// https://leetcode.com/problems/path-with-maximum-minimum-value/description/
pub fn maximum_minimum_path_i_dsu(grid: Vec<Vec<i32>>) -> i32 {
    struct UF {
        repr: Vec<usize>,
        size: Vec<usize>,
    }

    impl UF {
        fn new(n: usize) -> Self {
            let mut repr = vec![];
            for i in 0..n {
                repr.push(i);
            }
            Self {
                repr,
                size: vec![1; n],
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.repr[x] != x {
                self.repr[x] = self.find(self.repr[x]);
            }
            self.repr[x]
        }

        fn union(&mut self, x: usize, y: usize) {
            let (mut x, mut y) = (self.find(x), self.find(y));
            if x == y {
                return;
            }

            if self.size[x] > self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }

            self.repr[x] = y;
            self.size[y] += self.size[x];
        }
    }

    let row = grid.len();
    let col = grid[0].len();
    let mut visited = vec![vec![false; col]; row];

    let mut cells = vec![];
    for i in 0..row {
        for j in 0..col {
            cells.push((i, j, grid[i][j]));
        }
    }
    cells.sort_by_key(|e| e.2);
    cells.reverse();
    let mut uf = UF::new(row * col);

    for (x, y, val) in cells {
        visited[x][y] = true;
        let from = x * col + y;

        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0
                && nx < row as i32
                && ny >= 0
                && ny < col as i32
                && visited[nx as usize][ny as usize]
            {
                let to = nx as usize * col + ny as usize;
                uf.union(from, to);
                if uf.find(0) == uf.find(row * col - 1) {
                    return val;
                }
            }
        }
    }

    -1
}

pub fn maximum_minimum_path_ii_dijkstra(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;
    let (row, col) = (grid.len(), grid[0].len());

    let mut heap = BinaryHeap::new();
    heap.push((grid[0][0], 0, 0));

    let mut visited = vec![vec![false; col]; row];
    visited[0][0] = true;

    let mut min_val = i32::MAX;
    while let Some((val, x, y)) = heap.pop() {
        min_val = min_val.min(val);
        if x == row as i32 - 1 && y == col as i32 - 1 {
            return min_val;
        }

        for d in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x as i32 + d.0, y as i32 + d.1);
            if nx >= 0
                && ny >= 0
                && nx < row as i32
                && ny < col as i32
                && !visited[nx as usize][ny as usize]
            {
                visited[nx as usize][ny as usize] = true;
                heap.push((grid[nx as usize][ny as usize], nx, ny))
            }
        }
    }

    min_val
}
