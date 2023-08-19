pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    struct UF {
        repr: Vec<usize>,
        sz: Vec<i32>,
        max_size: i32,
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
                max_size: 1,
            }
        }
        fn find(&mut self, x: usize) -> usize {
            if self.repr[x] != x {
                self.repr[x] = self.find(self.repr[x]);
            }
            self.repr[x]
        }
        fn union(&mut self, x: usize, y: usize) -> bool {
            let mut x = self.find(x);
            let mut y = self.find(y);
            if x == y {
                return false;
            }
            if self.sz[x] < self.sz[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.repr[x] = y;
            self.sz[y] += self.sz[x];
            self.max_size = self.max_size.max(self.sz[y]);
            true
        }
    }

    let mut critical = vec![];
    let mut pseudo_critical = vec![];
    let mut new_edges = vec![];

    for (i, e) in edges.into_iter().enumerate() {
        new_edges.push(vec![e[0] as usize, e[1] as usize, e[2] as usize, i]);
    }
    new_edges.sort_by_key(|x| x[2]);

    let n = n as usize;
    let mut mst_weight = 0;

    let mut uf = UF::new(n);
    for i in 0..new_edges.len() {
        if uf.union(new_edges[i][0], new_edges[i][1]) {
            mst_weight += new_edges[i][2];
        }
    }

    for i in 0..new_edges.len() {
        let mut mst_weight_ignore = 0;
        let mut uf = UF::new(n);
        for j in 0..new_edges.len() {
            if i != j && uf.union(new_edges[j][0], new_edges[j][1]) {
                mst_weight_ignore += new_edges[j][2];
            }
        }

        if mst_weight_ignore > mst_weight || uf.max_size < n as i32 {
            critical.push(new_edges[i][3] as i32);
        } else {
            let mut mst_weight_forced = new_edges[i][2];
            let mut uf = UF::new(n);
            uf.union(new_edges[i][0], new_edges[i][1]);

            for j in 0..new_edges.len() {
                if i != j && uf.union(new_edges[j][0], new_edges[j][1]) {
                    mst_weight_forced += new_edges[j][2];
                }
            }

            if mst_weight_forced == mst_weight {
                pseudo_critical.push(new_edges[i][3] as i32);
            }
        }
    }
    vec![critical, pseudo_critical]
}

#[test]
fn test_critical() {
    let res = find_critical_and_pseudo_critical_edges(
        6,
        vec![
            vec![0, 1, 1],
            vec![1, 2, 1],
            vec![0, 2, 1],
            vec![2, 3, 4],
            vec![3, 4, 2],
            vec![3, 5, 2],
            vec![4, 5, 2],
        ],
    );
    println!("{:?}", res);
}
