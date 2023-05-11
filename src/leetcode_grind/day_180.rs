use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/description/
pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut freq1 = nums1.iter().copied().fold(HashMap::new(), |mut m, v| {
        *m.entry(v as i64 * v as i64).or_insert(0) += 1;
        m
    });
    let mut freq2 = nums2.iter().copied().fold(HashMap::new(), |mut m, v| {
        *m.entry(v as i64 * v as i64).or_insert(0) += 1;
        m
    });

    let mut ans = 0;
    for j in 0..nums2.len() {
        for k in j + 1..nums2.len() {
            let f = &(nums2[j] as i64 * nums2[k] as i64);
            if freq1.contains_key(f) {
                // println!("t1 f {} j {} k {} freq {}", f, j, k, freq1[f]);
                ans += freq1[f]
            }
        }
    }

    for j in 0..nums1.len() {
        for k in j + 1..nums1.len() {
            let f = &(nums1[j] as i64 * nums1[k] as i64);
            if freq2.contains_key(f) {
                // println!("t1 f {} j {} k {} freq {}", f, j, k, freq2[f]);
                ans += freq2[f]
            }
        }
    }
    ans
}

// https://leetcode.com/problems/uncrossed-lines/description/
pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (n1, n2) = (nums1.len(), nums2.len());
    let mut dp = vec![vec![0; n2 + 1]; n1 + 1];

    for i in 1..=n1 {
        for j in 1..=n2 {
            if nums1[i - 1] == nums2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
            }
        }
    }
    dp[n1][n2]
}

// https://leetcode.com/problems/find-peak-element/description/
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let (mut lo, mut hi) = (0, nums.len() - 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] > nums[mid + 1] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo as i32
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn search(nums: &Vec<i32>, target: i32, first: bool) -> i32 {
        let mut ans = -1;
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if first {
                if nums[mid] >= target {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                if nums[mid] <= target {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            if nums[mid] == target {
                ans = mid as i32;
            }
        }
        ans
    }

    vec![search(&nums, target, true), search(&nums, target, false)]
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
    let mut node = root.clone();
    let mut closest = root.as_ref().unwrap().borrow().val;
    while let Some(n) = node {
        if (n.borrow().val as f64 - target).abs() == (closest as f64 - target).abs()
            && n.borrow().val < closest
        {
            closest = n.borrow().val;
        } else if (n.borrow().val as f64 - target).abs() < (closest as f64 - target).abs() {
            closest = n.borrow().val;
        }
        if target < n.borrow().val as f64 {
            node = n.borrow().left.clone();
        } else {
            node = n.borrow().right.clone();
        }
    }
    closest
}

pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::{BTreeSet, HashMap};
    let mut edge_set = BTreeSet::new();
    for building in &buildings {
        let (left, right) = (building[0], building[1]);
        edge_set.insert(left);
        edge_set.insert(right);
    }
    let edges = edge_set.iter().copied().collect::<Vec<i32>>();
    let edge_idx_map = edge_set
        .into_iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, v)| {
            acc.insert(v, i);
            acc
        });
    let mut heights = vec![0; edge_idx_map.len()];
    for building in &buildings {
        let (left, right, height) = (building[0], building[1], building[2]);
        let (left_idx, right_idx) = (edge_idx_map[&left], edge_idx_map[&right]);

        for i in left_idx..right_idx {
            heights[i] = heights[i].max(height);
        }
    }

    let mut ans: Vec<Vec<i32>> = vec![];
    for (i, height) in heights.into_iter().enumerate() {
        let cur_pos = edges[i];

        if ans.is_empty() || ans.last().unwrap()[1] != height {
            ans.push(vec![cur_pos, height]);
        }
    }
    ans
}

// https://leetcode.com/problems/range-sum-query-mutable/
struct FenwickTree {
    tree: Vec<i32>,
    n: i32,
}
impl FenwickTree {
    fn new(n: i32) -> Self {
        Self {
            tree: vec![0; n as usize + 1],
            n,
        }
    }

    fn update(&mut self, mut index: i32, val: i32) {
        while index <= self.n {
            self.tree[index as usize] += val;
            index += index & -index;
        }
    }

    fn get_sum(&self, mut index: i32) -> i32 {
        let mut s = 0;
        while index >= 1 {
            s += self.tree[index as usize];
            index -= index & -index;
        }
        s
    }
}

struct NumArray {
    tree: FenwickTree,
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut tree = FenwickTree::new(nums.len() as i32 + 1);
        for (i, &n) in nums.iter().enumerate() {
            tree.update(i as i32 + 1, n);
        }
        Self { tree, nums }
    }

    fn update(&mut self, mut index: i32, val: i32) {
        let d = val - self.nums[index as usize];
        self.nums[index as usize] = val;
        self.tree.update(index + 1, d);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let rsum = self.tree.get_sum(right + 1);
        let lsum = self.tree.get_sum(left);
        rsum - lsum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test497() {
        println!("{:?}", search_range(vec![5, 7, 7, 8, 8, 10], 8));
    }

    #[test]
    fn test498() {
        println!(
            "{:?}",
            get_skyline(vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8]
            ])
        );
    }

    #[test]
    fn test499() {
        let mut rsq = NumArray::new(vec![1, 3, 5]);
        println!("{:?}", rsq.sum_range(0, 2)); // 9
        rsq.update(1, 2);
        println!("{:?}", rsq.sum_range(0, 2)); // 8
    }
}
