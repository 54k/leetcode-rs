// https://leetcode.com/problems/new-21-game/description/
pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    let mut dp = vec![0.; n as usize + 1];
    dp[0] = 1.;
    let mut s = if k > 0 { 1. } else { 0. };
    for i in 1..=n {
        dp[i as usize] = s / max_pts as f64;
        if i < k {
            s += dp[i as usize];
        }
        if i - max_pts >= 0 && i - max_pts < k {
            s -= dp[(i - max_pts) as usize];
        }
    }

    let mut ans = 0.;
    for i in k..=n {
        ans += dp[i as usize];
    }
    ans
}

// https://leetcode.com/problems/falling-squares/description/
pub fn falling_squares_sqare_root_decomposition(positions: Vec<Vec<i32>>) -> Vec<i32> {
    struct SqrtBlocks {
        heights: Vec<i32>,
        blocks: Vec<i32>,
        blocks_read: Vec<i32>,
        b: usize,
    }
    impl SqrtBlocks {
        fn new(t: usize) -> Self {
            let b = (t as f64).sqrt() as usize;
            Self {
                heights: vec![0; t],
                blocks: vec![0; b + 2],
                blocks_read: vec![0; b + 2],
                b,
            }
        }

        fn query(&mut self, mut left: i32, mut right: i32) -> i32 {
            let mut ans = 0;
            while left as usize % self.b > 0 && left <= right {
                ans = ans.max(self.heights[left as usize]);
                ans = ans.max(self.blocks[left as usize / self.b]);
                left += 1;
            }
            while right as usize % self.b != self.b - 1 && left <= right {
                ans = ans.max(self.heights[right as usize]);
                ans = ans.max(self.blocks[right as usize / self.b]);
                right -= 1;
            }
            while left <= right {
                ans = ans.max(self.blocks[left as usize / self.b]);
                ans = ans.max(self.blocks_read[left as usize / self.b]);
                left += self.b as i32;
            }
            ans
        }

        fn update(&mut self, mut left: i32, mut right: i32, h: i32) {
            while left as usize % self.b > 0 && left <= right {
                self.heights[left as usize] = h.max(self.heights[left as usize]);
                self.blocks_read[left as usize / self.b] =
                    h.max(self.blocks_read[left as usize / self.b]);
                left += 1;
            }
            while right as usize % self.b != self.b - 1 && left <= right {
                self.heights[right as usize] = h.max(self.heights[right as usize]);
                self.blocks_read[right as usize / self.b] =
                    h.max(self.blocks_read[right as usize / self.b]);
                right -= 1;
            }
            while left <= right {
                self.blocks[left as usize / self.b] = h.max(self.blocks[left as usize / self.b]);
                left += self.b as i32;
            }
        }
    }
    use std::collections::{BTreeSet, HashMap};
    // index compression
    let idx = positions
        .iter()
        .flat_map(|x| vec![x[0], x[0] + x[1] - 1])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    let mut sqrt_blocks = SqrtBlocks::new(idx.len());
    let mut best = 0;
    let mut ans = vec![];
    for pos in positions {
        let (left, right) = (idx[&pos[0]], idx[&(pos[0] + pos[1] - 1)]);
        let h = sqrt_blocks.query(left as i32, right as i32) + pos[1];
        sqrt_blocks.update(left as i32, right as i32, h);
        best = best.max(h);
        ans.push(best);
    }
    ans
}

pub fn falling_squares_segment_tree(positions: Vec<Vec<i32>>) -> Vec<i32> {
    struct SegmentTree {
        N: i32,
        H: i32,
        tree: Vec<i32>,
        lazy: Vec<i32>,
    }
    impl SegmentTree {
        fn new(N: i32) -> Self {
            let mut H = 1;
            while 1 << H < N {
                H += 1;
            }
            Self {
                N,
                H,
                tree: vec![0; 2 * N as usize],
                lazy: vec![0; N as usize],
            }
        }
        fn apply(&mut self, x: usize, val: i32) {
            self.tree[x] = self.tree[x].max(val);
            if x < self.N as usize {
                self.lazy[x] = self.lazy[x].max(val);
            }
        }
        fn pull(&mut self, mut x: usize) {
            while x > 1 {
                x >>= 1;
                self.tree[x] = self.tree[x * 2].max(self.tree[x * 2 + 1]);
                self.tree[x] = self.tree[x].max(self.lazy[x]);
            }
        }
        fn push(&mut self, x: usize) {
            for h in (1..=self.H).rev() {
                let y = x >> h;
                if self.lazy[y] > 0 {
                    self.apply(y * 2, self.lazy[y]);
                    self.apply(y * 2 + 1, self.lazy[y]);
                    self.lazy[y] = 0;
                }
            }
        }
        fn update(&mut self, mut left: i32, mut right: i32, h: i32) {
            let (l0, r0) = (left, right);
            left += self.N;
            right += self.N;
            while left <= right {
                if left & 1 == 1 {
                    self.apply(left as usize, h);
                    left += 1;
                }
                if right & 1 == 0 {
                    self.apply(right as usize, h);
                    right -= 1;
                }
                left >>= 1;
                right >>= 1;
            }
            self.pull(l0 as usize);
            self.pull(r0 as usize);
        }
        fn query(&mut self, mut left: i32, mut right: i32) -> i32 {
            let mut ans = 0;
            left += self.N;
            right += self.N;
            self.push(left as usize);
            self.push(right as usize);
            while left <= right {
                if left & 1 == 1 {
                    ans = ans.max(self.tree[left as usize]);
                    left += 1;
                }
                if right & 1 == 0 {
                    ans = ans.max(self.tree[right as usize]);
                    right -= 1;
                }
                left >>= 1;
                right >>= 1;
            }
            ans
        }
    }

    use std::collections::{BTreeSet, HashMap};
    // index compression
    let idx = positions
        .iter()
        .flat_map(|x| vec![x[0], x[0] + x[1] - 1])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    let mut tree = SegmentTree::new(idx.len() as i32);
    let mut best = 0;
    let mut ans = vec![];
    for pos in positions {
        let (left, right) = (idx[&pos[0]], idx[&(pos[0] + pos[1] - 1)]);
        let h = tree.query(left as i32, right as i32) + pos[1];
        tree.update(left as i32, right as i32, h);
        best = best.max(h);
        ans.push(best);
    }
    ans
}
