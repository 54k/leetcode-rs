// https://leetcode.com/problems/min-cost-to-connect-all-points/description
pub fn min_cost_connect_points_i(points: Vec<Vec<i32>>) -> i32 {
    struct UF {
        repr: Vec<usize>,
        size: Vec<usize>,
    }
    impl UF {
        fn new(n: usize) -> Self {
            let mut repr = vec![];
            for i in 0..n {
                repr.push(i);
            }
            UF {
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
            if self.size[x] > self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }

            self.repr[x] = y;
            self.size[y] += self.size[x];
            true
        }
    }

    let mut edges = vec![];
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let weight = (points[i][0] - points[j][0]).abs() + (points[j][1] - points[i][1]).abs();
            edges.push((weight, i, j));
        }
    }
    edges.sort();

    let mut total_cost = 0;
    let mut uf = UF::new(points.len());
    for (w, f, t) in edges {
        if uf.union(f, t) {
            total_cost += w;
        }
    }
    total_cost
}

pub fn min_cost_connect_points_ii(points: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();

    let mut total_cost = 0;
    let n = points.len();
    let mut used = 0;
    let mut in_mst = vec![false; n];

    heap.push(Reverse((0, 0)));

    while used < n {
        let Reverse((cur_weight, cur_node)) = heap.pop().unwrap();

        if in_mst[cur_node] {
            continue;
        }

        used += 1;
        in_mst[cur_node] = true;
        total_cost += cur_weight;

        for next_node in 0..n {
            if !in_mst[next_node] {
                let next_weight = (points[cur_node][0] - points[next_node][0]).abs()
                    + (points[cur_node][1] - points[next_node][1]).abs();
                heap.push(Reverse((next_weight, next_node)));
            }
        }
    }

    total_cost
}

pub fn min_cost_connect_points_iii(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut mst_cost = 0;
    let mut edges_used = 0;

    let mut in_mst = vec![false; n];

    let mut min_dist = vec![i32::MAX; n];
    min_dist[0] = 0;

    while edges_used < n {
        let mut curr_min_edge = i32::MAX;
        let mut curr_node = 0;

        for node in 0..n {
            if !in_mst[node] && curr_min_edge > min_dist[node] {
                curr_min_edge = min_dist[node];
                curr_node = node;
            }
        }

        mst_cost += curr_min_edge;
        edges_used += 1;
        in_mst[curr_node] = true;

        for next_node in 0..n {
            let dist = (points[curr_node][0] - points[next_node][0]).abs()
                + (points[curr_node][1] - points[next_node][1]).abs();
            if !in_mst[next_node] && min_dist[next_node] > dist {
                min_dist[next_node] = dist;
            }
        }
    }

    mst_cost
}

// https://leetcode.com/problems/sparse-matrix-multiplication/
pub fn multiply_i(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat1.len();
    let n = mat2[0].len();
    let mut result = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            for k in 0..mat2.len() {
                result[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }

    result
}

pub fn multiply_ii(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = mat1.len();
    let k = mat1[0].len();
    let m = mat2[0].len();
    let mut res = vec![vec![0; m]; n];

    for i in 0..n {
        for k in 0..k {
            if mat1[i][k] != 0 {
                for j in 0..m {
                    res[i][j] += mat1[i][k] * mat2[k][j];
                }
            }
        }
    }
    res
}

pub fn multiply_iii(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn compress_matrix_fun(mat: Vec<Vec<i32>>) -> Vec<Vec<(i32, usize)>> {
        let rows = mat.len();
        let cols = mat[0].len();
        let mut compressed_matrix = vec![];

        for r in 0..rows {
            let mut cur_row = vec![];
            for c in 0..cols {
                if mat[r][c] != 0 {
                    cur_row.push((mat[r][c], c));
                }
            }
            compressed_matrix.push(cur_row);
        }
        compressed_matrix
    }

    let m = mat1.len();
    let n = mat2[0].len();

    let a = compress_matrix_fun(mat1);
    let b = compress_matrix_fun(mat2);
    let mut res = vec![vec![0; n]; m];

    for mat1_row in 0..m {
        for mat1_elem in &a[mat1_row] {
            let elem1 = mat1_elem.0;
            let mat1_col = mat1_elem.1;

            for mat2_elem in &b[mat1_col] {
                let elem2 = mat2_elem.0;
                let mat2_col = mat2_elem.1;
                res[mat1_row][mat2_col] += elem1 * elem2;
            }
        }
    }

    res
}
