// https://leetcode.com/problems/design-graph-with-shortest-path-calculator/description
pub mod dijkstra_graph {
    struct Graph {
        adj: Vec<Vec<(usize, i32)>>,
    }

    impl Graph {
        fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
            let mut adj = vec![vec![]; n as usize];
            for e in edges {
                adj[e[0] as usize].push((e[1] as usize, e[2]));
            }
            Self { adj }
        }

        fn add_edge(&mut self, edge: Vec<i32>) {
            self.adj[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }

        fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
            use std::collections::BinaryHeap;
            const INF: i32 = 1_000_000_000;

            let mut heap = BinaryHeap::new();
            let mut vis = vec![false; self.adj.len()];
            let mut dist = vec![INF; self.adj.len()];

            heap.push((0, node1 as usize));
            dist[node1 as usize] = 0;

            while let Some((w, v)) = heap.pop() {
                let w = -w;

                if vis[v] {
                    continue;
                }
                vis[v] = true;

                for (u, d) in &self.adj[v] {
                    let u = *u;
                    let d = *d;
                    if dist[u] > w + d {
                        dist[u] = w + d;
                        heap.push((-dist[u], u));
                    }
                }
            }

            if dist[node2 as usize] >= INF {
                return -1;
            }
            dist[node2 as usize]
        }
    }
}

pub mod floyd_warhall {
    struct Graph {
        adj: Vec<Vec<i32>>,
    }

    impl Graph {
        fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
            const INF: i32 = 1_000_000_000;

            let n = n as usize;
            let mut adj = vec![vec![INF; n]; n];

            for e in edges {
                adj[e[0] as usize][e[1] as usize] = e[2];
            }

            for i in 0..n {
                adj[i][i] = 0;
            }

            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        adj[i][j] = adj[i][j].min(adj[i][k] + adj[k][j]);
                    }
                }
            }

            Self { adj }
        }

        fn add_edge(&mut self, edge: Vec<i32>) {
            for i in 0..self.adj.len() {
                for j in 0..self.adj.len() {
                    self.adj[i][j] = self.adj[i][j].min(
                        self.adj[i][edge[0] as usize] + self.adj[edge[1] as usize][j] + edge[2],
                    );
                }
            }
        }

        fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
            const INF: i32 = 1_000_000_000;

            let node1 = node1 as usize;
            let node2 = node2 as usize;

            if self.adj[node1][node2] >= INF {
                return -1;
            }
            self.adj[node1][node2]
        }
    }
}
