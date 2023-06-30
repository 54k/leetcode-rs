// https://leetcode.com/problems/last-day-where-you-can-still-cross/description/
pub fn latest_day_to_cross_bfs(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn can_cross(row: i32, col: i32, cells: &Vec<Vec<i32>>, day: usize) -> bool {
        use std::collections::VecDeque;

        let mut grid = vec![vec![0; col as usize]; row as usize];
        let mut queue = VecDeque::new();

        for i in 0..day {
            grid[cells[i][0] as usize - 1][cells[i][1] as usize - 1] = 1;
        }

        for i in 0..col {
            if grid[0][i as usize] == 0 {
                queue.push_back((0, i));
                grid[0][i as usize] = -1;
            }
        }

        while let Some((r, c)) = queue.pop_front() {
            if r == row - 1 {
                return true;
            }

            for d in &DIR {
                let (new_r, new_c) = (r + d.0, c + d.1);

                if 0 <= new_r
                    && new_r < row
                    && 0 <= new_c
                    && new_c < col
                    && grid[new_r as usize][new_c as usize] == 0
                {
                    grid[new_r as usize][new_c as usize] = -1;
                    queue.push_back((new_r, new_c));
                }
            }
        }

        false
    }

    let (mut left, mut right) = (1, row * col);

    while left < right {
        let mid = right - (right - left) / 2;

        if can_cross(row, col, &cells, mid as usize) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

pub fn latest_day_to_cross_dfs(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn can_cross(row: i32, col: i32, cells: &Vec<Vec<i32>>, day: usize) -> bool {
        let mut grid = vec![vec![0; col as usize]; row as usize];

        for i in 0..day {
            grid[cells[i][0] as usize - 1][cells[i][1] as usize - 1] = 1;
        }

        for i in 0..col {
            if grid[0][i as usize] == 0 && dfs(&mut grid, 0, i as i32, row, col) {
                return true;
            }
        }
        false
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32, row: i32, col: i32) -> bool {
        if r < 0 || r >= row || c < 0 || c >= col || grid[r as usize][c as usize] != 0 {
            return false;
        }

        if r == row - 1 {
            return true;
        }

        grid[r as usize][c as usize] = -1;

        for d in &DIR {
            let (new_r, new_c) = (r + d.0, c + d.1);
            if dfs(grid, new_r, new_c, row, col) {
                return true;
            }
        }
        return false;
    }

    let (mut left, mut right) = (1, row * col);

    while left < right {
        let mid = right - (right - left) / 2;

        if can_cross(row, col, &cells, mid as usize) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    struct DSU {
        root: Vec<usize>,
        size: Vec<usize>,
    }

    impl DSU {
        fn new(size: usize) -> Self {
            let mut root = vec![0; size];
            for i in 0..size {
                root[i] = i;
            }
            Self {
                root,
                size: vec![1; size],
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.root[x] != x {
                self.root[x] = self.find(self.root[x]);
            }
            self.root[x]
        }

        fn union(&mut self, x: usize, y: usize) {
            let (mut x, mut y) = (self.find(x), self.find(y));
            if x == y {
                return;
            }

            if self.size[x] < self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }

            self.root[x] = y;
            self.size[y] += self.size[x];
        }
    }

    let mut dsu = DSU::new(row as usize * col as usize + 2);
    let mut grid = vec![vec![0; col as usize]; row as usize];
    const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for i in (0..cells.len()).rev() {
        let (r, c) = (cells[i][0] as usize - 1, cells[i][1] as usize - 1);
        grid[r][c] = 1;
        let index_1 = r as i32 * col + c as i32 + 1;
        for d in &DIR {
            let (new_r, new_c) = (r as i32 + d.0, c as i32 + d.1);
            let index_2 = new_r * col + new_c + 1;

            if new_r >= 0
                && new_r < row
                && new_c >= 0
                && new_c < col
                && grid[new_r as usize][new_c as usize] == 1
            {
                dsu.union(index_1 as usize, index_2 as usize);
            }
        }

        if r == 0 {
            dsu.union(0, index_1 as usize);
        }
        if r as usize == row as usize - 1 {
            dsu.union(row as usize * col as usize + 1, index_1 as usize);
        }
        if dsu.find(0) == dsu.find(row as usize * col as usize + 1) {
            return i as i32;
        }
    }
    -1
}
