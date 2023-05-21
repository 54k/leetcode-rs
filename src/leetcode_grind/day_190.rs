// https://leetcode.com/problems/shortest-bridge/description/
pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    fn dfs(grid: &mut Vec<Vec<i32>>, bfs_queue: &mut Vec<(i32, i32)>, x: i32, y: i32, n: i32) {
        grid[x as usize][y as usize] = 2;
        bfs_queue.push((x, y));

        for dir in DIRS {
            let (new_x, new_y) = (x + dir.0, y + dir.1);
            if 0 <= new_x
                && new_x < n
                && 0 <= new_y
                && new_y < n
                && grid[new_x as usize][new_y as usize] == 1
            {
                dfs(grid, bfs_queue, new_x, new_y, n)
            }
        }
    }

    let n = grid.len();
    let (mut first_x, mut first_y) = (-1, -1);

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                first_x = i as i32;
                first_y = j as i32;
                break;
            }
        }
    }

    let n = n as i32;
    let mut bfs_queue = vec![];
    dfs(&mut grid, &mut bfs_queue, first_x, first_y, n);
    let mut distance = 0;

    while !bfs_queue.is_empty() {
        let mut new_bfs = vec![];

        for (x, y) in &bfs_queue {
            for dir in DIRS {
                let (new_x, new_y) = (*x + dir.0, *y + dir.1);
                if 0 <= new_x && new_x < n && 0 <= new_y && new_y < n {
                    if grid[new_x as usize][new_y as usize] == 1 {
                        return distance;
                    } else if grid[new_x as usize][new_y as usize] == 0 {
                        new_bfs.push((new_x, new_y));
                        grid[new_x as usize][new_y as usize] = -1;
                    }
                }
            }
        }

        bfs_queue = new_bfs;
        distance += 1;
    }

    distance
}
