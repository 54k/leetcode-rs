// https://leetcode.com/problems/stone-game-ii/description/
pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    fn rec(piles: &Vec<i32>, dp: &mut Vec<Vec<Vec<i32>>>, p: usize, i: usize, m: usize) -> i32 {
        if i == piles.len() {
            return 0;
        }
        if dp[p][i][m] != -1 {
            return dp[p][i][m];
        }
        let mut res = if p == 1 { 1000000 } else { -1 };
        let mut s = 0;

        for x in 1..=(2 * m).min(piles.len() - i) {
            s += piles[i + x - 1];
            if p == 0 {
                res = res.max(s + rec(piles, dp, 1, i + x, m.max(x)));
            } else {
                res = res.min(rec(piles, dp, 0, i + x, m.max(x)));
            }
        }
        dp[p][i][m] = res;
        dp[p][i][m]
    }
    let mut dp = vec![vec![vec![-1; piles.len() + 1]; piles.len() + 1]; 2];
    rec(&piles, &mut dp, 0, 0, 1)
}

// https://leetcode.com/problems/range-sum-query-mutable/description/
mod rmq_spatial_tree {
    pub struct NumArray {
        tree: Vec<i32>,
        N: usize,
    }
    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let mut s = Self {
                tree: vec![0; nums.len() * 2],
                N: nums.len(),
            };
            for (i, num) in nums.into_iter().enumerate() {
                s.update(i as i32, num);
            }
            s
        }

        pub fn update(&mut self, index: i32, val: i32) {
            let mut index = index as usize + self.N;
            self.tree[index] = val;
            while index > 0 {
                index /= 2;
                self.tree[index] = self.tree[index * 2] + self.tree[index * 2 + 1];
            }
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            let mut left = left as usize + self.N;
            let mut right = right as usize + self.N;
            let mut sum = 0;
            while left <= right {
                if left & 1 == 1 {
                    sum += self.tree[left];
                    left += 1;
                }
                if right & 1 == 0 {
                    sum += self.tree[right];
                    right -= 1;
                }
                left >>= 1;
                right >>= 1;
            }
            sum
        }
    }
}

mod rmq_fenwick_tree {
    pub struct NumArray {
        tree: Vec<i32>,
        nums: Vec<i32>,
    }
    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let mut s = Self {
                tree: vec![0; nums.len() + 1],
                nums: vec![0; nums.len()],
            };
            for (i, num) in nums.into_iter().enumerate() {
                s.update(i as i32, num);
            }
            s
        }

        pub fn update(&mut self, mut index: i32, val: i32) {
            let d = val - self.nums[index as usize];
            self.nums[index as usize] = val;
            index += 1;

            while (index as usize) < self.tree.len() {
                self.tree[index as usize] += d;
                index += index & -index;
            }
        }

        fn sum(&self, mut index: i32) -> i32 {
            let mut sum = 0;
            while index >= 1 {
                sum += self.tree[index as usize];
                index -= index & -index;
            }
            sum
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.sum(right + 1) - self.sum(left)
        }
    }
}

mod rmq_sqrt_decompose {
    pub struct NumArray {
        blocks: Vec<i32>,
        nums: Vec<i32>,
        N: usize,
    }

    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let N = (nums.len() as f64).sqrt().ceil() as usize;
            let mut s = Self {
                blocks: vec![0; N],
                nums,
                N,
            };
            for (i, &num) in s.nums.iter().enumerate() {
                s.blocks[(i / N) % N] += num;
            }
            s
        }

        pub fn update(&mut self, index: i32, val: i32) {
            let d = val - self.nums[index as usize];
            self.nums[index as usize] = val;
            self.blocks[(index as usize / self.N) % self.N] += d;
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            let mut s = 0;
            let (mut left, mut right) = (left as usize, right as usize);
            while left <= right && left % self.N != 0 {
                s += self.nums[left];
                left += 1;
            }
            while left <= right && right % self.N != self.N - 1 {
                s += self.nums[right];
                right -= 1;
            }
            while left <= right {
                s += self.blocks[(left / self.N) % self.N];
                left += self.N;
            }
            s
        }
    }
}

// https://leetcode.com/problems/range-sum-query-2d-mutable/description/
mod rmq2d {
    struct NumMatrix {}

    // impl NumMatrix {
    //     fn new(matrix: Vec<Vec<i32>>) -> Self {}

    //     fn update(&self, row: i32, col: i32, val: i32) {}

    //     fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {}
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test507() {
        let numarr = rmq_sqrt_decompose::NumArray::new(vec![1, 3, 5]);
        println!("{}", numarr.sum_range(0, 2));
        let numarr = rmq_spatial_tree::NumArray::new(vec![1, 3, 5]);
        println!("{}", numarr.sum_range(0, 2));
    }
}
