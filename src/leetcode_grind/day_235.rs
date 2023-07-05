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
pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
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

// https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/description/
pub fn minimum_hamming_distance(
    source: Vec<i32>,
    target: Vec<i32>,
    allowed_swaps: Vec<Vec<i32>>,
) -> i32 {
    todo!()
}
