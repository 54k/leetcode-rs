// Do a breadth-first search, where the "nodes" are actually (Node, color of last edge taken).
// https://leetcode.com/problems/shortest-path-with-alternating-colors/description/
pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    use std::collections::*;

    let mut adj = vec![vec![]; n as usize];
    for red_edge in red_edges {
        adj[red_edge[0] as usize].push((red_edge[1] as usize, 0));
    }
    for blue_edge in blue_edges {
        adj[blue_edge[0] as usize].push((blue_edge[1] as usize, 1));
    }

    let mut visited = vec![vec![false; 2]; n as usize];
    let mut distances = vec![-1; n as usize];

    let mut q = VecDeque::new();
    q.push_back((0, 0, 0)); // node, color, dist
    q.push_back((0, 1, 0)); // node, color, dist

    distances[0] = 0;
    visited[0][0] = true;
    visited[0][1] = true;

    while let Some((from, from_color, dist)) = q.pop_front() {
        for (to, to_color) in &adj[from] {
            if !visited[*to][*to_color] && from_color != *to_color {
                if distances[*to] == -1 {
                    distances[*to] = (dist + 1) as i32;
                }
                visited[*to][*to_color] = true;
                q.push_back((*to, *to_color, dist + 1));
            }
        }
    }

    distances
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test220() {
        println!(
            "{:?}",
            shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![])
        ); // [0,1,-1]

        println!(
            "{:?}",
            shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]])
        ); // [0,1,-1]
    }
}
