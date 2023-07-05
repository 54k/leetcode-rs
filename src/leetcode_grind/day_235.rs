// https://leetcode.com/problems/graph-valid-tree/description/
pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    struct UF {
        repr: Vec<usize>,
        sz: Vec<i32>,
        cmp: usize,
    }

    impl UF {
        fn new(n: usize) -> Self {
            let mut repr = vec![];
            for i in 0..n {
                repr.push(i);
            }

            Self {
                repr,
                sz: vec![1; n],
                cmp: n,
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.repr[x] != x {
                self.repr[x] = self.find(self.repr[x])
            }
            self.repr[x]
        }

        fn union(&mut self, x: usize, y: usize) {
            let (mut x, mut y) = (self.find(x), self.find(y));
            if x == y {
                return;
            }
            if self.sz[x] < self.sz[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.repr[y] = x;
            self.sz[x] += self.sz[y];
            self.cmp -= 1;
        }

        fn is_connected(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
    }

    let mut uf = UF::new(n as usize);
    for edge in edges {
        let (x, y) = (edge[0], edge[1]);
        if uf.is_connected(x as usize, y as usize) {
            return false;
        }
        uf.union(x as usize, y as usize)
    }
    uf.cmp == 1
}

// https://leetcode.com/problems/the-earliest-moment-when-everyone-become-friends/description/
pub fn earliest_acq(mut logs: Vec<Vec<i32>>, n: i32) -> i32 {
    struct UF {
        repr: Vec<usize>,
        sz: Vec<i32>,
        cmp: i32,
    }
    impl UF {
        fn new(n: i32) -> Self {
            let mut repr = vec![];
            for i in 0..n as usize {
                repr.push(i);
            }
            Self {
                repr,
                sz: vec![1; n as usize],
                cmp: n,
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.repr[x] != x {
                self.repr[x] = self.find(self.repr[x]);
            }
            self.repr[x]
        }

        fn union(&mut self, x: usize, y: usize) {
            let (mut x, mut y) = (self.find(x), self.find(y));
            if x == y {
                return;
            }
            if self.sz[x] < self.sz[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.repr[y] = x;
            self.sz[x] += self.sz[y];
            self.cmp -= 1;
        }
    }

    let mut uf = UF::new(n);
    logs.sort();
    for log in logs {
        let (ts, x, y) = (log[0], log[1], log[2]);
        uf.union(x as usize, y as usize);
        if uf.cmp == 1 {
            return ts;
        }
    }
    -1
}

// https://leetcode.com/problems/smallest-string-with-swaps/description/
pub fn smallest_string_with_swaps_dfs(s: String, pairs: Vec<Vec<i32>>) -> String {
    fn dfs(
        s: &Vec<char>,
        vertex: usize,
        chars: &mut Vec<char>,
        indices: &mut Vec<usize>,
        seen: &mut Vec<bool>,
        adj: &Vec<Vec<usize>>,
    ) {
        chars.push(s[vertex]);
        indices.push(vertex);

        seen[vertex] = true;

        for &a in &adj[vertex] {
            if !seen[a] {
                dfs(s, a, chars, indices, seen, adj);
            }
        }
    }

    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();

    let mut adj = vec![vec![]; n];
    for pair in pairs {
        let (x, y) = (pair[0] as usize, pair[1] as usize);
        adj[x].push(y);
        adj[y].push(x);
    }

    let mut seen = vec![false; n];
    let mut answer = vec!['a'; n];

    for vertex in 0..n {
        if !seen[vertex] {
            let mut chars = vec![];
            let mut indices = vec![];

            dfs(&s, vertex, &mut chars, &mut indices, &mut seen, &mut adj);

            chars.sort();
            indices.sort();

            for index in 0..chars.len() {
                answer[indices[index]] = chars[index];
            }
        }
    }

    answer.into_iter().collect()
}

pub fn smallest_string_with_swaps_dsu(s: String, pairs: Vec<Vec<i32>>) -> String {
    use std::collections::HashMap;

    struct UF {
        root: Vec<usize>,
        rank: Vec<i32>,
    }

    impl UF {
        fn new(n: usize) -> Self {
            let mut root = vec![];
            for i in 0..n {
                root.push(i);
            }
            Self {
                root,
                rank: vec![1; n],
            }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.root[x] != x {
                self.root[x] = self.find(self.root[x]);
            }
            self.root[x]
        }

        fn union(&mut self, x: usize, y: usize) {
            let (mut x, mut y) = (self.find(x), self.find(y));
            if x == y {
                return;
            }
            if self.rank[x] < self.rank[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.root[y] = x;
            self.rank[x] += self.rank[y];
        }
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut uf = UF::new(s.len());

    for edge in pairs {
        let (source, destination) = (edge[0] as usize, edge[1] as usize);
        uf.union(source, destination);
    }

    let mut root_to_component = HashMap::new();

    for vertex in 0..s.len() {
        let root = uf.find(vertex);
        root_to_component.entry(root).or_insert(vec![]).push(vertex);
    }

    let mut smallest_string = vec!['a'; s.len()];

    for indices in root_to_component.values() {
        let mut chars = vec![];
        for &idx in indices {
            chars.push(s[idx]);
        }
        chars.sort();

        for idx in 0..indices.len() {
            smallest_string[indices[idx]] = chars[idx];
        }
    }

    smallest_string.into_iter().collect()
}
