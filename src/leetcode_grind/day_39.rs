#[allow(dead_code)]
pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    fn bfs(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        fn bfs(e: usize, adj: &Vec<Vec<usize>>, colors: &mut Vec<i32>) -> bool {
            use std::collections::VecDeque;
            let mut q = VecDeque::new();
            q.push_back(e);
            colors[e] = 0;

            while !q.is_empty() {
                let v = q.pop_front().unwrap();

                for n in adj[v].iter() {
                    if colors[*n] == colors[v] {
                        return false;
                    }
                    if colors[*n] == -1 {
                        colors[*n] = 1 - colors[v];
                        q.push_back(*n);
                    }
                }
            }
            true
        }

        let mut adj = vec![vec![]; n as usize + 1];
        for d in dislikes {
            adj[d[0] as usize].push(d[1] as usize);
            adj[d[1] as usize].push(d[0] as usize);
        }
        let mut colors = vec![-1; n as usize + 1];
        for e in 1..=n as usize {
            if colors[e] == -1 && !bfs(e as usize, &adj, &mut colors) {
                return false;
            }
        }
        true
    }

    fn dfs(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        fn dfs(e: usize, color: i32, adj: &Vec<Vec<usize>>, colors: &mut Vec<i32>) -> bool {
            colors[e] = color;
            for n in adj[e].iter() {
                if colors[*n] == colors[e] {
                    return false;
                }
                if colors[*n] == -1 && !dfs(*n, 1 - color, adj, colors) {
                    return false;
                }
            }
            true
        }

        let mut adj = vec![vec![]; n as usize + 1];
        for d in dislikes {
            adj[d[0] as usize].push(d[1] as usize);
            adj[d[1] as usize].push(d[0] as usize);
        }
        let mut colors = vec![-1; n as usize + 1];
        for e in 1..=n as usize {
            if colors[e] == -1 && !dfs(e as usize, 0, &adj, &mut colors) {
                return false;
            }
        }
        true
    }

    fn uf(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        struct UF {
            c: Vec<usize>,
        }

        impl UF {
            fn new(n: usize) -> Self {
                let mut c = vec![0; n];
                for (i, v) in c.iter_mut().enumerate() {
                    *v = i;
                }
                UF { c }
            }

            fn find(&mut self, x: usize) -> usize {
                if self.c[x] == x {
                    return x;
                }
                self.c[x] = self.find(self.c[x]);
                self.c[x]
            }

            fn union(&mut self, x: usize, y: usize) {
                let px = self.find(x);
                let py = self.find(y);
                self.c[px] = self.c[py];
            }
        }

        let mut adj = vec![vec![]; n as usize + 1];
        for dis in dislikes {
            adj[dis[0] as usize].push(dis[1] as usize);
            adj[dis[1] as usize].push(dis[0] as usize);
        }

        let mut uf = UF::new(n as usize + 1);

        for e in 1..=n as usize {
            for neighbor in adj[e].iter() {
                if uf.find(e) == uf.find(*neighbor) {
                    return false;
                }
                uf.union(adj[e][0], *neighbor);
            }
        }

        true
    }

    uf(n, dislikes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test123() {
        println!(
            "{}",
            possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]])
        ); // true
        println!(
            "{}",
            possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]])
        ); // false
    }
}
