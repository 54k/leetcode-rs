// https://leetcode.com/problems/falling-squares/
// https://leetcode.com/problems/falling-squares/editorial/
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    struct H(Vec<i32>);
    impl H {
        fn update(&mut self, l: usize, r: usize, x: i32) {
            for i in l..=r {
                self.0[i] = x;
            }
        }
        fn query(&self, l: usize, r: usize) -> i32 {
            let mut max = -1;
            for i in l..=r {
                max = max.max(self.0[i]);
            }
            max
        }
    }
    use std::collections::{BTreeSet, HashMap};
    let mut set = BTreeSet::new();
    for p in &positions {
        set.insert(p[0]);
        set.insert(p[0] + p[1] - 1);
    }
    let idx = set
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<HashMap<i32, usize>>();
    let mut heights = H(vec![0; idx.len()]);
    let mut best = 0;
    let mut ans = vec![];
    for pos in positions {
        let (left, right) = (idx[&pos[0]], idx[&(pos[0] + pos[1] - 1)]);
        let h = heights.query(left, right) + pos[1];
        heights.update(left, right, h);
        best = best.max(h);
        ans.push(best);
    }
    ans
}

// https://leetcode.com/problems/maximum-subsequence-score/description/
pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let k = k as usize;
    let mut arr = vec![];
    for i in 0..nums1.len() {
        arr.push((nums1[i], nums2[i]));
    }
    arr.sort_by_key(|x| x.1);
    arr.reverse();
    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    let mut ans = 0;
    for (val, min) in arr {
        sum += val as i64;
        heap.push(-val);
        if heap.len() > k {
            sum -= -heap.pop().unwrap() as i64;
        }
        if heap.len() == k {
            ans = ans.max(sum * min as i64);
        }
    }
    ans
}
