// https://leetcode.com/problems/kth-ancestor-of-a-tree-node/description/
struct TreeAncestor {
    a: Vec<Vec<i32>>,
    max_d: usize,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut max_d = 0;
        while (1 << max_d) <= n as usize {
            max_d += 1;
        }

        let mut a = vec![vec![-1; max_d]; n as usize];

        for i in 0..max_d {
            for j in 0..n as usize {
                if i == 0 {
                    a[j][i] = parent[j];
                } else if a[j][i - 1] != -1 {
                    a[j][i] = a[a[j][i - 1] as usize][i - 1];
                }
            }
        }

        TreeAncestor { a, max_d }
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node;
        for i in 0..self.max_d {
            if ((1 << i) & k as usize) != 0 {
                node = self.a[node as usize][i];
                if node == -1 {
                    break;
                }
            }
        }
        node
    }
}
