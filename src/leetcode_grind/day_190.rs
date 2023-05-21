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

// https://leetcode.com/problems/the-skyline-problem/description/
pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    pub fn get_skyline_brute_sweep_line(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::BTreeSet;
        let mut edges = BTreeSet::new();

        for building in &buildings {
            let (left, right) = (building[0], building[1]);
            edges.insert(left);
            edges.insert(right);
        }

        let positions = edges.into_iter().collect::<Vec<_>>();
        let mut ans: Vec<Vec<i32>> = vec![];

        for position in &positions {
            let mut max_height = 0;
            for building in &buildings {
                let (left, right, height) = (building[0], building[1], building[2]);

                if left <= *position && *position < right {
                    max_height = max_height.max(height);
                }
            }

            // If its the first key point or the height changes,
            // we add [position, maxHeight] to 'answer'.
            if ans.is_empty() || ans.last().unwrap()[1] != max_height {
                ans.push(vec![*position, max_height]);
            }
        }

        ans
    }

    pub fn get_skyline_line_sweep_bin_heap(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;
        let mut edges = vec![];
        for (i, building) in buildings.iter().enumerate() {
            let (left, right) = (building[0], building[1]);
            edges.push((left, i));
            edges.push((right, i));
        }

        edges.sort_by_key(|x| x.0);
        let mut live = BinaryHeap::new();
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut idx = 0;

        while idx < edges.len() {
            let curr_x = edges[idx].0;

            while idx < edges.len() && edges[idx].0 == curr_x {
                // The index 'b' of this building in 'buildings'
                let b = edges[idx].1;

                if buildings[b][0] == curr_x {
                    let (right, height) = (buildings[b][1], buildings[b][2]);
                    live.push((height, right));
                }
                idx += 1;
            }

            // If the tallest live building has been passed,
            // we remove it from 'live'.
            while !live.is_empty() && live.peek().unwrap().1 <= curr_x {
                live.pop();
            }

            // Get the maximum height from 'live'.
            let curr_height = live.peek().unwrap_or(&(0, 0)).0;

            // If the height changes at this currX, we add this
            // skyline key point [currX, max_height] to 'answer'.
            if ans.is_empty() || ans.last().unwrap()[1] != curr_height {
                ans.push(vec![curr_x, curr_height]);
            }
        }
        ans
    }

    get_skyline_line_sweep_bin_heap(buildings)
}
