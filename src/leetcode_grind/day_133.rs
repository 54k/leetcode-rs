// https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/description/
// https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/editorial/
pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    fn make_adj(n: i32, edges: &Vec<Vec<i32>>) -> (Vec<Vec<usize>>, Vec<bool>) {
        let mut adj = vec![vec![]; n as usize];
        for e in edges {
            let from = e[0] as usize;
            let to = e[1] as usize;
            adj[from].push(to);
            adj[to].push(from);
        }
        (adj, vec![false; n as usize])
    }
    fn using_dfs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        fn dfs(v: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> i64 {
            let mut count = 1;
            visited[v] = true;
            for &u in &adj[v] {
                if !visited[u] {
                    count += dfs(u, adj, visited);
                }
            }
            count
        }
        let (adj, mut visited) = make_adj(n, &edges);
        let mut pairs_count = 0;
        let mut num_of_components = n as i64;
        for v in 0..n as usize {
            if !visited[v] {
                let size_of_component = dfs(v, &adj, &mut visited);
                pairs_count += size_of_component * (num_of_components - size_of_component);
                num_of_components -= size_of_component;
            }
        }
        pairs_count
    }
    fn using_bfs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        fn bfs(v: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> i64 {
            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            queue.push_back(v);
            visited[v] = true;
            let mut count = 0;
            while let Some(v) = queue.pop_front() {
                count += 1;
                for &u in &adj[v] {
                    if !visited[u] {
                        visited[u] = true;
                        queue.push_back(u);
                    }
                }
            }
            count
        }
        let (adj, mut visited) = make_adj(n, &edges);
        let mut pairs_count = 0;
        let mut num_of_components = n as i64;
        for v in 0..n as usize {
            if !visited[v] {
                let size_of_component = bfs(v, &adj, &mut visited);
                pairs_count += size_of_component * (num_of_components - size_of_component);
                num_of_components -= size_of_component;
            }
        }
        pairs_count
    }
    fn using_union_find(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        use std::collections::HashMap;
        struct UnionFind {
            links: Vec<usize>,
            sz: Vec<i32>,
        }
        impl UnionFind {
            fn new(n: usize) -> Self {
                let links = (0..n).collect::<Vec<_>>();
                Self {
                    links,
                    sz: vec![1; n],
                }
            }
            fn find(&mut self, x: usize) -> usize {
                if self.links[x] != x {
                    self.links[x] = self.find(self.links[x]);
                }
                self.links[x]
            }
            fn union(&mut self, x: usize, y: usize) {
                let mut x = self.find(x);
                let mut y = self.find(y);
                if self.sz[x] > self.sz[y] {
                    std::mem::swap(&mut x, &mut y);
                }
                self.links[x] = y;
                self.sz[y] += self.sz[x];
            }
        }
        let mut uf = UnionFind::new(n as usize);
        for e in edges {
            let from = e[0] as usize;
            let to = e[1] as usize;
            uf.union(from, to);
        }
        let mut components = HashMap::new();
        for i in 0..n as usize {
            let parent = uf.find(i);
            *components.entry(parent).or_insert(0i64) += 1;
        }
        let mut count = 0;
        let mut remaining = n as i64;
        for &sz in components.values() {
            count += sz * (remaining - sz);
            remaining -= sz;
        }
        count
    }
    using_union_find(n, edges)
}

// https://leetcode.com/problems/minimize-the-difference-between-target-and-chosen-elements/description/
pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
    let mut dp = vec![vec![false; 5000]; mat.len() + 1];
    dp[0][0] = true;
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            for k in 0..dp[i].len() {
                if dp[i][k] {
                    dp[i + 1][k + mat[i][j] as usize] = true;
                }
            }
        }
    }
    let mut min_diff = i32::MAX;
    for i in 0..5000 {
        if dp[mat.len()][i] {
            min_diff = min_diff.min((target - i as i32).abs())
        }
    }
    min_diff
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test374() {
        println!(
            "{}",
            count_pairs(
                7,
                vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]]
            )
        ); // 14
    }

    #[test]
    fn test375() {
        println!(
            "{}",
            minimize_the_difference(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13)
        ); // 0
        println!("{}", minimize_the_difference(vec![vec![1, 2, 9, 8, 7]], 6)); // 1
        println!(
            "{}",
            minimize_the_difference(vec![vec![1], vec![2], vec![3]], 100)
        ); // 94
    }
}
