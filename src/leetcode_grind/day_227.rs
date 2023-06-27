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

// https://leetcode.com/problems/destroy-sequential-targets/description/
pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
    todo!()
}

// https://leetcode.com/problems/minimum-falling-path-sum-ii/
pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-v/description/
pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vi/description/
pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vii/description/
pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/range-sum-query-2d-mutable/description/
mod rmq2d {
    struct NumMatrix {}

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            todo!()
        }

        fn update(&self, row: i32, col: i32, val: i32) {
            todo!()
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            todo!()
        }
    }
}

// https://leetcode.com/problems/maximize-grid-happiness/description/
pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    todo!()
}
