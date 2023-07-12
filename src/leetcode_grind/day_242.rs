// https://leetcode.com/problems/find-eventual-safe-states/
pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    pub fn eventual_safe_nodes_dfs_topo(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(
            node: usize,
            adj: &Vec<Vec<usize>>,
            visit: &mut Vec<bool>,
            in_stack: &mut Vec<bool>,
        ) -> bool {
            if in_stack[node] {
                return true;
            }

            if visit[node] {
                return false;
            }

            visit[node] = true;
            in_stack[node] = true;

            for &neighbor in &adj[node] {
                if dfs(neighbor, adj, visit, in_stack) {
                    return true;
                }
            }

            in_stack[node] = false;
            false
        }

        let n = graph.len();
        let mut adj = vec![vec![]; n];
        for i in 0..n {
            for &node in &graph[i] {
                adj[i].push(node as usize);
            }
        }

        let mut visit = vec![false; n];
        let mut in_stack = vec![false; n];

        for i in 0..n {
            dfs(i, &adj, &mut visit, &mut in_stack);
        }

        let mut safe_nodes = vec![];
        for i in 0..n {
            if !in_stack[i] {
                safe_nodes.push(i as i32);
            }
        }
        safe_nodes
    }
    pub fn eventual_safe_nodes_khan_topo(graph: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;

        let n = graph.len();
        let mut in_degree = vec![0; n];
        let mut adj = vec![vec![]; n];

        for i in 0..n {
            for &node in &graph[i] {
                adj[node as usize].push(i);
                in_degree[i] += 1;
            }
        }

        let mut q = VecDeque::new();

        for i in 0..n {
            if in_degree[i] == 0 {
                q.push_back(i);
            }
        }

        let mut safe = vec![false; n];

        while let Some(node) = q.pop_front() {
            safe[node] = true;

            for &neighbour in &adj[node] {
                in_degree[neighbour] -= 1;
                if in_degree[neighbour] == 0 {
                    q.push_back(neighbour);
                }
            }
        }

        let mut safe_nodes = vec![];
        for i in 0..n {
            if safe[i] {
                safe_nodes.push(i as i32);
            }
        }

        safe_nodes
    }
    eventual_safe_nodes_khan_topo(graph)
}
