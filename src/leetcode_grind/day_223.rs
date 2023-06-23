// https://leetcode.com/problems/longest-arithmetic-subsequence/
pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut max_len = 0;
    let mut dp = vec![HashMap::new(); nums.len()];

    for right in 0..nums.len() {
        dp[right] = HashMap::new();
        for left in 0..right {
            let diff = nums[left] - nums[right];
            let l = *dp[left].get(&diff).unwrap_or(&1) + 1;
            dp[right].insert(diff, l);
            max_len = max_len.max(dp[right][&diff]);
        }
    }
    max_len
}

// https://leetcode.com/problems/longest-arithmetic-subsequence/
pub fn destroy_targets(mut nums: Vec<i32>, space: i32) -> i32 {
    use std::collections::HashMap;
    let mut max = i32::MIN;
    let mut hm = HashMap::new();
    for i in 0..nums.len() {
        let k = nums[i] % space;
        *hm.entry(k).or_insert(0) += 1;
        if hm[&k] > max {
            max = hm[&k];
        }
    }
    let mut ans = i32::MAX;
    for i in 0..nums.len() {
        let k = nums[i] % space;
        if hm[&k] == max && ans > nums[i] {
            ans = nums[i];
        }
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
