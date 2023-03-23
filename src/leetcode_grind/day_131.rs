// https://leetcode.com/problems/number-of-operations-to-make-network-connected/description/
pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    fn make_adj(n: i32, connections: &Vec<Vec<i32>>) -> (Vec<Vec<usize>>, Vec<bool>) {
        let mut ans = vec![vec![]; n as usize];
        for c in connections {
            ans[c[0] as usize].push(c[1] as usize);
            ans[c[1] as usize].push(c[0] as usize);
        }
        (ans, vec![false; n as usize])
    }
    fn using_dfs(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        fn dfs(v: usize, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
            visited[v] = true;
            for &u in &adj[v] {
                if !visited[u] {
                    dfs(u, visited, adj)
                }
            }
        }
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let (adj, mut visited) = make_adj(n, &connections);
        let mut cmp_count = 0;
        for v in 0..n as usize {
            if !visited[v] {
                cmp_count += 1;
                dfs(v, &mut visited, &adj);
            }
        }
        cmp_count - 1
    }
    fn using_bfs(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        fn bfs(v: usize, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            queue.push_back(v);
            visited[v] = true;
            while let Some(n) = queue.pop_front() {
                for &u in &adj[n] {
                    if !visited[u] {
                        visited[u] = true;
                        queue.push_back(u);
                    }
                }
            }
        }
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let (adj, mut visited) = make_adj(n, &connections);
        let mut cmp_count = 0;
        for v in 0..n as usize {
            if !visited[v] {
                cmp_count += 1;
                bfs(v, &mut visited, &adj);
            }
        }
        cmp_count - 1
    }
    fn using_union_find(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        struct UnionFind {
            links: Vec<usize>,
            size: Vec<i32>,
        }
        impl UnionFind {
            fn new(n: usize) -> Self {
                let links = (0..n).collect::<Vec<_>>();
                Self {
                    links,
                    size: vec![1; n],
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
                if self.size[x] > self.size[y] {
                    std::mem::swap(&mut x, &mut y);
                }
                self.links[x] = y;
                self.size[y] += self.size[x];
            }
        }
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let mut union_find = UnionFind::new(n as usize);
        let mut num_of_components = n;
        for connection in connections {
            let from = connection[0] as usize;
            let to = connection[1] as usize;
            if union_find.find(from) != union_find.find(to) {
                num_of_components -= 1;
                union_find.union(from, to);
            }
        }
        num_of_components - 1
    }
    using_union_find(n, connections)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test365() {
        println!(
            "{}",
            make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]])
        ); // 1
    }
}
