use crate::leetcode_grind::day_85::find_anagrams;

// https://leetcode.com/problems/number-of-provinces/description/
pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    fn dfs(v: usize, seen: &mut [bool], is_connected: &Vec<Vec<i32>>) {
        for u in 0..is_connected.len() {
            if is_connected[v][u] == 1 && !seen[u] {
                seen[u] = true;
                dfs(u, seen, is_connected);
            }
        }
    }
    let mut seen = vec![false; is_connected.len()];
    let mut ans = 0;
    for i in 0..is_connected.len() {
        if !seen[i] {
            seen[i] = true;
            dfs(i, &mut seen, &is_connected);
            ans += 1;
        }
    }
    ans
}

// https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/description/
pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    fn bfs(v: usize, adj: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back(v);
        while let Some(v) = queue.pop_front() {
            seen[v] = true;
            for &u in &adj[v] {
                if !seen[u] {
                    queue.push_back(u);
                }
            }
        }
    }
    let mut adj = vec![vec![]; n as usize];
    for e in edges {
        let from = e[0] as usize;
        let to = e[1] as usize;
        adj[from].push(to);
        adj[to].push(from);
    }
    let mut seen = vec![false; n as usize];
    let mut ans = 0;
    for i in 0..n as usize {
        if !seen[i] {
            seen[i] = true;
            bfs(i, &adj, &mut seen);
        }
    }
    ans
}

// https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/
pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    edges
        .into_iter()
        .fold(vec![0; n as usize], |mut acc, v| {
            acc[v[1] as usize] += 1;
            acc
        })
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| if x == 0 { Some(i as i32) } else { None })
        .collect()
}

// https://leetcode.com/problems/max-area-of-island/description/
pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut ans = 0;
    let mut stack = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                let mut area = 0;
                stack.push((i as i32, j as i32));
                while let Some((x, y)) = stack.pop() {
                    if grid[x as usize][y as usize] == 1 {
                        grid[x as usize][y as usize] = 0;
                        area += 1;
                        ans = ans.max(area);
                        for dir in DIRS {
                            let ux = x + dir.0;
                            let uy = y + dir.1;
                            if ux < 0
                                || ux >= grid.len() as i32
                                || uy < 0
                                || uy >= grid[0].len() as i32
                                || grid[ux as usize][uy as usize] == 0
                            {
                                continue;
                            }
                            stack.push((ux, uy));
                        }
                    }
                }
            }
        }
    }
    ans
}

// https://leetcode.com/problems/reachable-nodes-with-restrictions/description/
pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let mut adj = vec![vec![]; n as usize];
    for e in edges {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }
    let mut visited = vec![false; n as usize];
    visited[0] = true;
    for r in restricted {
        visited[r as usize] = true;
    }
    let mut ans = 0;
    let mut stack = vec![];
    stack.push(0);
    while let Some(v) = stack.pop() {
        ans += 1;
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                stack.push(u);
            }
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test434() {
        println!(
            "{}",
            find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]])
        );
    }

    #[test]
    fn test435() {
        println!(
            "{}",
            max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            ])
        ); // 6
    }

    #[test]
    fn test436() {
        println!(
            "{}",
            reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![3, 1],
                    vec![4, 0],
                    vec![0, 5],
                    vec![5, 6]
                ],
                vec![4, 5]
            )
        ); // 4
    }
}
