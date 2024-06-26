use std::cmp::min;

// https://leetcode.com/problems/shortest-path-in-binary-matrix/
pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][0] == 1 || grid[grid.len() - 1][grid[0].len() - 1] == 1 {
        return -1;
    }
    fn is_valid(x: i32, y: i32, grid: &Vec<Vec<i32>>) -> bool {
        x >= 0
            && x < grid.len() as i32
            && y >= 0
            && y < grid[0].len() as i32
            && grid[x as usize][y as usize] == 0
    }
    use std::collections::VecDeque;
    const DIR: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (-1, -1),
    ];
    let mut queue = VecDeque::new();
    grid[0][0] = 1;
    queue.push_back((0, 0, 1));
    while let Some((x, y, dist)) = queue.pop_front() {
        if x == grid.len() as i32 - 1 && y == grid[0].len() as i32 - 1 {
            return dist;
        }
        for (nx, ny) in DIR {
            let (ux, uy) = (x + nx, y + ny);
            if is_valid(ux, uy, &grid) {
                queue.push_back((ux, uy, dist + 1));
                grid[ux as usize][uy as usize] = 1;
            }
        }
    }
    -1
}

// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    fn is_valid(x: i32, y: i32, grid: &Vec<Vec<i32>>) -> bool {
        x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
    }
    use std::collections::VecDeque;
    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visited = vec![vec![vec![false; k as usize + 1]; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, k, 0)); // x, y, remain, steps
    visited[0][0][k as usize] = true;
    while let Some((x, y, remain, steps)) = queue.pop_front() {
        if x == grid.len() as i32 - 1 && y == grid[0].len() as i32 - 1 {
            return steps;
        }
        for (nx, ny) in DIR {
            let (ux, uy) = (x + nx, y + ny);
            if is_valid(ux, uy, &grid) {
                if grid[ux as usize][uy as usize] == 0
                    && !visited[ux as usize][uy as usize][remain as usize]
                {
                    visited[ux as usize][uy as usize][remain as usize] = true;
                    queue.push_back((ux, uy, remain, steps + 1));
                } else if remain > 0 && !visited[ux as usize][uy as usize][remain as usize - 1] {
                    visited[ux as usize][uy as usize][remain as usize - 1] = true;
                    queue.push_back((ux, uy, remain - 1, steps + 1));
                }
            }
        }
    }
    -1
}

// https://leetcode.com/problems/shortest-distance-from-all-buildings/description/
pub fn shortest_distance(mut grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut total = vec![vec![0; grid[0].len()]; grid.len()];
    let mut level = 0;
    let mut min_dist = i32::MAX;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                min_dist = i32::MAX;
                let mut steps = 0;
                let mut queue = VecDeque::new();
                queue.push_back((i as i32, j as i32));
                while !queue.is_empty() {
                    steps += 1;
                    for _ in 0..queue.len() {
                        let (x, y) = queue.pop_front().unwrap();
                        for (nx, ny) in DIR {
                            let (ux, uy) = (x + nx, y + ny);
                            if ux >= 0
                                && ux < grid.len() as i32
                                && uy >= 0
                                && uy < grid[0].len() as i32
                                && grid[ux as usize][uy as usize] == level
                            {
                                total[ux as usize][uy as usize] += steps;
                                grid[ux as usize][uy as usize] -= 1;
                                min_dist = min_dist.min(total[ux as usize][uy as usize]);

                                queue.push_back((ux, uy));
                            }
                        }
                    }
                }
                level -= 1;
            }
        }
    }
    if min_dist == i32::MAX {
        -1
    } else {
        min_dist
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test437() {
        println!(
            "{}",
            shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]])
        ); // 2
    }

    #[test]
    fn test438() {
        println!(
            "{}",
            shortest_path(
                vec![
                    vec![0, 0, 0],
                    vec![1, 1, 0],
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 0]
                ],
                1
            )
        ); // 6
    }
}
