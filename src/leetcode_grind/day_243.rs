// https://leetcode.com/problems/min-cost-to-connect-all-points/description/
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    pub fn min_cost_connect_points_kruskal(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = vec![];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                edges.push((
                    i,
                    j,
                    (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(),
                ));
            }
        }

        edges.sort_by_key(|x| x.2);

        struct UF {
            repr: Vec<usize>,
            size: Vec<i32>,
        }

        impl UF {
            fn new(n: usize) -> Self {
                let mut repr = vec![0; n];
                for i in 0..n {
                    repr[i] = i;
                }
                Self {
                    repr,
                    size: vec![1; n],
                }
            }

            fn find(&mut self, x: usize) -> usize {
                if self.repr[x] != x {
                    self.repr[x] = self.find(self.repr[x]);
                }
                self.repr[x]
            }

            fn union(&mut self, x: usize, y: usize) -> bool {
                let (mut x, mut y) = (self.find(x), self.find(y));
                if x == y {
                    return false;
                }

                if self.size[x] < self.size[y] {
                    std::mem::swap(&mut x, &mut y);
                }

                self.repr[y] = x;
                self.size[x] += self.size[y];
                true
            }
        }

        let mut uf = UF::new(n);
        let mut ans = 0;
        for e in edges {
            if uf.union(e.0, e.1) {
                ans += e.2;
            }
        }
        ans
    }

    pub fn min_cost_connect_points_prims(points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = points.len();
        let mut in_mst = vec![false; n];

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));

        let mut mst_cost = 0;
        let mut edges_used = 0;

        while edges_used < n {
            let Reverse(top) = heap.pop().unwrap();
            let (weight, curr_node) = (top.0, top.1);

            if in_mst[curr_node] {
                continue;
            }

            in_mst[curr_node] = true;

            mst_cost += weight;
            edges_used += 1;

            for next_node in 0..n {
                if !in_mst[next_node] {
                    let next_weight = (points[curr_node][0] - points[next_node][0]).abs()
                        + (points[curr_node][1] - points[next_node][1]).abs();
                    heap.push(Reverse((next_weight, next_node)));
                }
            }
        }

        mst_cost as i32
    }

    min_cost_connect_points_prims(points)
}
