use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/unique-binary-search-trees-ii/description/
pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;

    let n = n as usize;
    let mut dp: Vec<Vec<Vec<Node>>> = vec![vec![vec![]; n + 1]; n + 1];

    for i in 1..=n {
        dp[i][i] = vec![Some(Rc::new(RefCell::new(TreeNode {
            val: i as i32,
            left: None,
            right: None,
        })))];
    }

    for num_of_nodes in 2..=n {
        for start in 1..=n - num_of_nodes + 1 {
            let end = start + num_of_nodes - 1;

            for i in start..=end {
                let mut left_subtrees = if i != start {
                    dp[start][i - 1].clone()
                } else {
                    vec![]
                };
                if left_subtrees.is_empty() {
                    left_subtrees.push(None);
                }

                let mut right_subtrees = if i != end {
                    dp[i + 1][end].clone()
                } else {
                    vec![]
                };
                if right_subtrees.is_empty() {
                    right_subtrees.push(None);
                }

                for left in left_subtrees.into_iter() {
                    for right in right_subtrees.clone() {
                        let tree_node = TreeNode {
                            val: i as i32,
                            left: left.clone(),
                            right,
                        };

                        dp[start][end].push(Some(Rc::new(RefCell::new(tree_node))));
                    }
                }
            }
        }
    }

    dp[1][n].clone()
}

// https://leetcode.com/problems/range-sum-query-mutable/description/
mod fenwick {
    struct Fenwick {
        tree: Vec<i32>,
        n: i32,
    }

    impl Fenwick {
        fn new(n: i32) -> Self {
            Self {
                tree: vec![0; n as usize + 1],
                n,
            }
        }

        fn update(&mut self, mut i: i32, val: i32) {
            while i <= self.n {
                self.tree[i as usize] += val;
                i += i & -i;
            }
        }

        fn query(&mut self, mut i: i32) -> i32 {
            let mut sum = 0;
            while i >= 1 {
                sum += self.tree[i as usize];
                i -= i & -i;
            }
            sum
        }
    }

    struct NumArray {
        f: Fenwick,
        nums: Vec<i32>,
    }

    impl NumArray {
        fn new(nums: Vec<i32>) -> Self {
            let mut f = Fenwick::new(nums.len() as i32 + 1);
            for (i, v) in nums.iter().enumerate() {
                f.update(i as i32 + 1, *v);
            }
            Self { f, nums }
        }

        fn update(&mut self, index: i32, val: i32) {
            let diff = val - self.nums[index as usize];
            self.nums[index as usize] = val;
            self.f.update(index + 1, diff)
        }

        fn sum_range(&mut self, left: i32, right: i32) -> i32 {
            self.f.query(right + 1) - self.f.query(left)
        }
    }
}

mod fenwick_2d {
    struct Fenwick {
        bit: Vec<Vec<i32>>,
        rows: i32,
        cols: i32,
    }

    impl Fenwick {
        fn new(rows: i32, cols: i32) -> Self {
            Self {
                bit: vec![vec![0; cols as usize + 1]; rows as usize + 1],
                rows,
                cols,
            }
        }

        fn update(&mut self, r: i32, c: i32, val: i32) {
            let mut row = r;
            while row <= self.rows {
                let mut col = c;
                while col <= self.cols {
                    self.bit[row as usize][col as usize] += val;
                    col += col & -col;
                }
                row += row & -row;
            }
        }

        fn query(&self, r: i32, c: i32) -> i32 {
            let mut sum = 0;
            let mut row = r;
            while row > 0 {
                let mut col = c;
                while col > 0 {
                    sum += self.bit[row as usize][col as usize];
                    col -= col & -col;
                }
                row -= row & -row;
            }
            sum
        }
    }

    struct NumMatrix {
        fen: Fenwick,
        matrix: Vec<Vec<i32>>,
    }

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            let rows = matrix.len();
            let cols = matrix[0].len();

            let mut fen = Fenwick::new(rows as i32, cols as i32);
            for r in 0..rows {
                for c in 0..cols {
                    fen.update(r as i32 + 1, c as i32 + 1, matrix[r][c]);
                }
            }

            Self { fen, matrix }
        }

        fn update(&mut self, mut row: i32, mut col: i32, val: i32) {
            let old_val = self.sum_region(row, col, row, col);
            println!("old_val {}", old_val);
            row += 1;
            col += 1;
            let diff = val - old_val;
            self.fen.update(row, col, diff);
        }

        fn sum_region(&self, mut row1: i32, mut col1: i32, mut row2: i32, mut col2: i32) -> i32 {
            row1 += 1;
            col1 += 1;
            row2 += 1;
            col2 += 1;
            let a = self.fen.query(row2, col2);
            let b = self.fen.query(row1 - 1, col1 - 1);
            let c = self.fen.query(row2, col1 - 1);
            let d = self.fen.query(row1 - 1, col2);
            println!("a{} b{} c{} d{}", a, b, c, d);
            return (a + b) - (c + d);
        }
    }

    #[test]
    fn test_fenwick_2d() {
        let mut f2d = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);

        let sum = f2d.sum_region(2, 1, 4, 3);
        println!("{}", sum); // 8
    }
}
