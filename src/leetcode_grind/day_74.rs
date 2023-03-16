pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
    const INF: i32 = 1000000007;

    fn bfs(adj: &Vec<Vec<usize>>, s: usize) -> Vec<i32> {
        use std::collections::*;
        let mut visited = vec![false; adj.len()];
        let mut dist = vec![INF; adj.len()];
        let mut q = VecDeque::new();
        visited[s] = true;
        dist[s] = 0;
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for &u in &adj[v] {
                if !visited[u] {
                    dist[u] = dist[v] + 1;
                    visited[u] = true;
                    q.push_back(u);
                }
            }
        }
        dist
    }

    fn dfs(adj: &Vec<Vec<usize>>, s: usize) -> Vec<i32> {
        fn rec(s: usize, adj: &[Vec<usize>], visited: &mut [bool], dist: &mut [i32]) {
            visited[s] = true;
            for &u in &adj[s] {
                if !visited[u] {
                    dist[u] = dist[s] + 1;
                    rec(u, adj, visited, dist);
                }
            }
        }
        let mut visited = vec![false; adj.len()];
        let mut dist = vec![INF; adj.len()];
        dist[s] = 0;
        rec(s, adj, &mut visited, &mut dist);
        dist
    }

    let mut adj = vec![vec![]; edges.len()];
    for (v, &u) in edges.iter().enumerate() {
        if u >= 0 {
            adj[v].push(u as usize);
        }
    }

    let dist1 = dfs(&adj, node1 as usize);
    let dist2 = dfs(&adj, node2 as usize);

    let mut ans = (INF, -1);

    for i in 0..edges.len() {
        ans = ans.min((dist1[i].max(dist2[i]), i as i32));
    }
    ans.1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test176() {
        println!("{}", closest_meeting_node(vec![2, 2, 3, -1], 0, 1)); // 2
        println!("{}", closest_meeting_node(vec![1, 2, -1], 0, 2)); // 2
        println!("{}", closest_meeting_node(vec![5, 4, 5, 4, 3, 6, -1], 0, 1)); // -1
    }
}
