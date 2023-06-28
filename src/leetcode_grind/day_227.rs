// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/description/
pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
    use std::collections::{BinaryHeap, HashSet};
    let (m, n) = (nums1.len(), nums2.len());

    let mut ans = vec![];
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    heap.push((-(nums1[0] + nums2[0]), 0, 0));
    visited.insert((0, 0));

    while k > 0 && !heap.is_empty() {
        k -= 1;
        let (_, i, j) = heap.pop().unwrap();
        ans.push(vec![nums1[i], nums2[j]]);

        if i + 1 < m && !visited.contains(&(i + 1, j)) {
            heap.push((-(nums1[i + 1] + nums2[j]), i + 1, j));
            visited.insert((i + 1, j));
        }

        if j + 1 < n && !visited.contains(&(i, j + 1)) {
            heap.push((-(nums1[i] + nums2[j + 1]), i, j + 1));
            visited.insert((i, j + 1));
        }
    }

    ans
}

// https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/description/
pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    use std::collections::HashMap;

    let mut dp = HashMap::new();
    let mut ans = 1;

    for a in arr {
        let before_a = *dp.get(&(a - difference)).unwrap_or(&0);
        dp.insert(a, before_a + 1);
        ans = ans.max(dp[&a]);
    }

    ans
}
