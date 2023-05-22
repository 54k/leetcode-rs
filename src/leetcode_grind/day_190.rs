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
    pub fn get_skyline_brute(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::{BTreeSet, HashMap};
        let mut edges = BTreeSet::new();
        for b in &buildings {
            edges.insert(b[0]);
            edges.insert(b[1]);
        }
        let edges = edges.into_iter().collect::<Vec<_>>();
        let mut edge_idx_map = HashMap::new();
        for (i, &e) in edges.iter().enumerate() {
            edge_idx_map.insert(e, i);
        }
        let mut heights = vec![0; edges.len()];
        for b in &buildings {
            let (left, right, height) = (b[0], b[1], b[2]);
            let (left_idx, right_idx) = (edge_idx_map[&left], edge_idx_map[&right]);
            for i in left_idx..right_idx {
                heights[i] = heights[i].max(height);
            }
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        for (i, h) in heights.into_iter().enumerate() {
            if ans.is_empty() || ans.last().unwrap()[1] != h {
                ans.push(vec![edges[i], h]);
            }
        }
        ans
    }

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

    pub fn get_skyline_line_sweep_two_heaps(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;
        let mut edges = vec![];
        for i in 0..buildings.len() {
            edges.push((buildings[i][0], buildings[i][2]));
            edges.push((buildings[i][1], -buildings[i][2]));
        }
        edges.sort_by_key(|x| x.0);

        let mut live = BinaryHeap::new();
        let mut past = BinaryHeap::new();

        let mut ans: Vec<Vec<i32>> = vec![];

        let mut idx = 0;
        while idx < edges.len() {
            let curr_x = edges[idx].0;

            while idx < edges.len() && edges[idx].0 == curr_x {
                let height = edges[idx].1;
                if height > 0 {
                    live.push(height);
                } else {
                    past.push(-height);
                }
                idx += 1;
            }

            while !past.is_empty() && live.peek().unwrap() == past.peek().unwrap() {
                live.pop();
                past.pop();
            }

            let curr_height = *live.peek().unwrap_or(&0);

            if ans.is_empty() || ans.last().unwrap()[1] != curr_height {
                ans.push(vec![curr_x, curr_height]);
            }
        }
        ans
    }

    pub fn get_skyline_union_find(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        struct UF {
            root: Vec<usize>,
        }
        impl UF {
            fn new(n: usize) -> Self {
                let mut root = vec![];
                for i in 0..n {
                    root.push(i);
                }
                Self { root }
            }
            fn find(&mut self, x: usize) -> usize {
                if self.root[x] != x {
                    self.root[x] = self.find(self.root[x]);
                }
                self.root[x]
            }
            fn union(&mut self, x: usize, y: usize) {
                let (x, y) = (self.find(x), self.find(y));
                self.root[x] = self.root[y];
            }
        }

        use std::collections::{BTreeSet, HashMap};

        let mut set = BTreeSet::new();
        for building in &buildings {
            set.insert(building[0]);
            set.insert(building[1]);
        }
        let edges = set.into_iter().collect::<Vec<_>>();

        let mut edge_idx_map = HashMap::new();
        for i in 0..edges.len() {
            edge_idx_map.insert(edges[i], i);
        }
        buildings.sort_by_key(|x| x[2]);
        let buildings = buildings.into_iter().rev().collect::<Vec<Vec<i32>>>();

        let n = edges.len();
        let mut uf = UF::new(n);
        let mut heights = vec![0; n];

        for building in &buildings {
            let (left_edge, right_edge) = (building[0], building[1]);
            let height = building[2];
            let (mut left_idx, right_idx) = (edge_idx_map[&left_edge], edge_idx_map[&right_edge]);

            // While we haven't update the the root of 'left_idx':
            while left_idx < right_idx {
                // Find the root of left index 'left_idx', that is:
                // The rightmost index having the same height as 'left_idx'.
                left_idx = uf.find(left_idx);
                // If left_idx < right_idx, we have to update both the root and height
                // of 'left_idx', and move on to the next index towards 'right_idx'.
                // That is: increment 'left_idx' by 1.
                if left_idx < right_idx {
                    uf.union(left_idx, right_idx);
                    heights[left_idx] = height;
                    left_idx += 1;
                }
            }
        }

        let mut ans = vec![];
        // Finally, we just need to iterate over updated heights, and
        // add every skyline key point to 'answer'.
        for i in 0..n {
            if i == 0 || heights[i - 1] != heights[i] {
                ans.push(vec![edges[i], heights[i]]);
            }
        }
        ans
    }

    get_skyline_line_sweep_two_heaps(buildings)
}
